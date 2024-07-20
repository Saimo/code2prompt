use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let template_dir = Path::new(&home_dir).join(".code2prompt").join("templates");

    // Create the template directory if it doesn't exist
    fs::create_dir_all(&template_dir).expect("Failed to create template directory");

    // Copy all template files from the project's templates directory to ~/.code2prompt/templates
    let project_template_dir = Path::new("templates");
    if project_template_dir.is_dir() {
        for entry in fs::read_dir(project_template_dir).expect("Failed to read templates directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "hbs") {
                let dest_path = template_dir.join(path.file_name().unwrap());
                fs::copy(&path, &dest_path).expect("Failed to copy template file");
            }
        }
    }

    // Copy the default template
    let default_template = include_str!("src/default_template.hbs");
    fs::write(template_dir.join("default_template.hbs"), default_template)
        .expect("Failed to write default template");

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=src/default_template.hbs");
}