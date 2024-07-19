// src/config.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use log::info;
use crate::cli::Cli;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigStore {
    configs: HashMap<String, Vec<String>>,
}

impl ConfigStore {
    pub fn save_or_set(path: &Path, args:Cli, amount: usize) -> Result<Cli> {
        let mut config = ConfigStore::load()?;
        let args_serialized = serde_json::to_string(&args)?;

        if amount > 2 {
            info!("Too many arguments passed. Ignoring the extra arguments.");
            config.set_args_for_path(path, vec![args_serialized])?;
            config.save()?;
            return Ok(args);
        }


        if config.args_exist_for_path(path)? {
            let existing_args = config.get_args_for_path(path).unwrap().clone();
            let existing_args_serialized = existing_args[0].clone();
            let existing_args_deserialized: Cli = serde_json::from_str(&existing_args_serialized)?;
            Ok(existing_args_deserialized)
        } else {
            config.set_args_for_path(path, vec![args_serialized])?;
            config.save()?;
            Ok(args)
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = ConfigStore::get_config_path()?;
        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            serde_json::from_str(&config_str)
                .context("Failed to parse config file")
        } else {
            Ok(ConfigStore::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = ConfigStore::get_config_path()?;
        let config_str = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&config_path, config_str)
            .context("Failed to write config file")?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .context("Failed to get home directory")?;
        Ok(home_dir.join(".code2prompt_configs.json"))
    }

    fn args_exist_for_path(&self, path: &Path) -> Result<bool> {
        let canonical_path = path.canonicalize()?;
        Ok(self.configs.contains_key(&canonical_path.to_string_lossy().to_string()))
    }

    pub fn get_args_for_path(&self, path: &Path) -> Option<&Vec<String>> {
        let canonical_path = path.canonicalize().ok()?;
        self.configs.get(&canonical_path.to_string_lossy().to_string())
    }

    pub fn set_args_for_path(&mut self, path: &Path, args: Vec<String>) -> Result<()> {
        let canonical_path = path.canonicalize()?;

        if !canonical_path.exists() {
            return Err(anyhow::anyhow!("Path does not exist"));
        }

        self.configs.insert(canonical_path.to_string_lossy().to_string(), args);
        Ok(())
    }
}