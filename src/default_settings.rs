use directories::ProjectDirs;

use dialoguer::Confirm;
use fs_extra::{copy_items, dir};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;

// #[derive(Embed)]
// #[folder = "$CARGO_MANIFEST_DIR/assets/"]

// struct Asset;

pub fn ask_for_config_creation(proj_dirs: ProjectDirs) {
    let confirmation = Confirm::new()
        .with_prompt(
            "Do you want to create the default configuration files in the appropriate location?",
        )
        .interact()
        .unwrap();

    if confirmation {
        // create base config path
        if create_dir(proj_dirs.config_dir()) {
            copy_file(Path::new("./assets/nerdcli.toml"), proj_dirs.config_dir())
        }
        // create quotes path
        let quote_path = proj_dirs.config_dir().join("quotes/en");
        if create_dir(&quote_path) {
            copy_file(
                Path::new("./assets/quotes/en/computer-science.toml"),
                &quote_path,
            );
            copy_file(
                Path::new("./assets/quotes/en/science-fiction.toml"),
                &quote_path,
            )
        }
        // create image path
        let images_path = proj_dirs.config_dir().join("images/mountains");
        if create_dir(&images_path) {
            copy_file(
                Path::new("./assets/images/mountains/verbier.png"),
                &images_path,
            )
        }

        println!(
            "\nThe configuration can be found under: {}",
            proj_dirs.config_dir().as_os_str().to_str().unwrap()
        )
    } else {
        println!("May the force be with you.");
    }
}

fn copy_file(source: &Path, target: &Path) {
    println!(
        "copy from {} to {}",
        source.as_os_str().to_str().unwrap(),
        target.as_os_str().to_str().unwrap()
    );

    let options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();
    from_paths.push(source);
    match copy_items(&from_paths, target, &options) {
        Ok(_) => println!("...done"),
        Err(e) => {
            eprintln!("Unable to copy file: {}", e);
            exit(1);
        }
    }
}

fn create_dir(path: &Path) -> bool {
    match create_dir_all(path) {
        Ok(_) => {
            return true;
        }
        Err(e) => {
            eprint!("Error: Could not create config directories: {}", e);
            return false;
        }
    };
}
