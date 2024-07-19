//! code2prompt is a command-line tool to generate an LLM prompt from a codebase directory.
//!
//! Author: Mufeed VH (@mufeedvh)
//! Contributor: Olivier D'Ancona (@ODAncona)

use anyhow::{Context, Result};
use clap::Parser;
use code2prompt::{
    copy_to_clipboard, get_model_info, get_tokenizer, get_git_diff, get_git_diff_between_branches, get_git_log,
    handle_undefined_variables, handlebars_setup, label, render_template, traverse_directory, write_to_file,
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use serde_json::json;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use code2prompt::cli::Cli;
use code2prompt::config::ConfigStore;

// Constants
const DEFAULT_TEMPLATE_NAME: &str = "default";
const CUSTOM_TEMPLATE_NAME: &str = "custom";

fn main() -> Result<()> {
    // get how many args are passed
    
    env_logger::init();
    let cli = Cli::parse();
    let amount =  std::env::args().len();
    let args = ConfigStore::save_or_set(&cli.path, cli.clone(), amount)?;
    
    // Handlebars Template Setup
    let (template_content, template_name) = get_template(&args)?;
    let handlebars = handlebars_setup(&template_content, template_name)?;

    // Progress Bar Setup
    let spinner = setup_spinner("Traversing directory and building tree...");

    // Parse Patterns
    let include_patterns = parse_patterns(&args.include);
    let exclude_patterns = parse_patterns(&args.exclude);

    let (tree, files) = traverse_directory(
        &args.path,
        &include_patterns,
        &exclude_patterns,
        args.include_priority,
        args.line_number,
        args.relative_paths,
        args.exclude_from_tree,
        args.no_codeblock,
        args.remove_comments,
    )?;

    if args.remove_comments {
        let total_comments_removed: usize = files
            .iter()
            .filter_map(|file| file["comments_removed"].as_u64())
            .sum::<u64>() as usize;

        println!(
            "{}{}{} {}",
            "[".bold().white(),
            "i".bold().blue(),
            "]".bold().white(),
            format!("Total comment lines removed: {}", total_comments_removed).blue()
        );
    }

    // Git Diff
    let git_diff = if args.diff {
        spinner.set_message("Generating git diff...");
        get_git_diff(&args.path).unwrap_or_default()
    } else {
        String::new()
    };

    // git diff two get_git_diff_between_branches
    let mut git_diff_branch: String = String::new();
    if let Some(branches) = &args.git_diff_branch {
        spinner.set_message("Generating git diff between two branches...");
        let branches = parse_patterns(&Some(branches.to_string()));
        if branches.len() != 2 {
            error!("Please provide exactly two branches separated by a comma.");
            std::process::exit(1);
        }
        git_diff_branch = get_git_diff_between_branches(&args.path, &branches[0], &branches[1]).unwrap_or_default()
    }

    // git diff two get_git_diff_between_branches
    let mut git_log_branch: String = String::new();
    if let Some(branches) = &args.git_log_branch {
        spinner.set_message("Generating git log between two branches...");
        let branches = parse_patterns(&Some(branches.to_string()));
        if branches.len() != 2 {
            error!("Please provide exactly two branches separated by a comma.");
            std::process::exit(1);
        }
        git_log_branch = get_git_log(&args.path, &branches[0], &branches[1]).unwrap_or_default()
    }

    spinner.finish_with_message("Done!".green().to_string());
    
    // Prepare JSON Data
    let mut data = json!({
        "absolute_code_path": label(&args.path),
        "source_tree": tree,
        "files": files,
        "git_diff": git_diff,
        "git_diff_branch": git_diff_branch,
        "git_log_branch": git_log_branch
    });

    debug!(
        "JSON Data: {}",
        serde_json::to_string_pretty(&data).unwrap()
    );

    // Handle undefined variables
    handle_undefined_variables(&mut data, &template_content)?;

    // Render the template
    let rendered = render_template(&handlebars, template_name, &data)?;

    // Display Token Count
    let token_count = if args.tokens {
        let bpe = get_tokenizer(&args.encoding);
        bpe.encode_with_special_tokens(&rendered).len()
    } else {
        0
    };

    let paths: Vec<String> = files.iter()
        .filter_map(|file| file.get("path").and_then(|p| p.as_str()).map(|s| s.to_string()))
        .collect();

    let model_info = get_model_info(&args.encoding);

    if args.json {
        let json_output = json!({
            "prompt": rendered,
            "directory_name": label(&args.path),
            "token_count": token_count,
            "model_info": model_info,
            "files": paths,
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    } else {
        if args.tokens {
            println!(
                "{}{}{} Token count: {}, Model info: {}",
                "[".bold().white(),
                "i".bold().blue(),
                "]".bold().white(),
                token_count.to_string().bold().yellow(),
                model_info
            );
        }
    }

    // Copy to Clipboard
    if !args.no_clipboard {
        copy_to_clipboard(&rendered)?;
    }

    // Output File
    if let Some(output_path) = &args.output {
        write_to_file(output_path, &rendered)?;
    }

    Ok(())
}

/// Sets up a progress spinner with a given message
///
/// # Arguments
///
/// * `message` - A message to display with the spinner
///
/// # Returns
///
/// * `ProgressBar` - The configured progress spinner
fn setup_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner
}

/// Parses comma-separated patterns into a vector of strings
///
/// # Arguments
///
/// * `patterns` - An optional string containing comma-separated patterns
///
/// # Returns
///
/// * `Vec<String>` - A vector of parsed patterns
fn parse_patterns(patterns: &Option<String>) -> Vec<String> {
    match patterns {
        Some(patterns) if !patterns.is_empty() => {
            patterns.split(',').map(|s| s.trim().to_string()).collect()
        }
        _ => vec![],
    }
}

/// Retrieves the template content and name based on the CLI arguments
///
/// # Arguments
///
/// * `args` - The parsed CLI arguments
///
/// # Returns
///
/// * `Result<(String, &str)>` - A tuple containing the template content and name
fn get_template(args: &Cli) -> Result<(String, &str)> {
    if let Some(template_path) = &args.template {
        let content = std::fs::read_to_string(template_path)
            .context("Failed to read custom template file")?;
        Ok((content, CUSTOM_TEMPLATE_NAME))
    } else {
        Ok((
            include_str!("default_template.hbs").to_string(),
            DEFAULT_TEMPLATE_NAME,
        ))
    }
}
