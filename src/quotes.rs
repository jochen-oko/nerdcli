use std::{
    fs::{self, metadata},
    io,
    path::{Path, PathBuf},
};

use crate::settings::NerdcliConfig;
use serde_derive::Deserialize;
use toml;

use rand::prelude::*;

#[derive(Deserialize, Default)]
pub struct Quotes {
    pub quotes: Vec<Quote>,
}

#[derive(Deserialize, Default, Clone)]
pub struct Quote {
    pub text: String,
    pub author: String,
    pub source: Option<String>,
    pub date: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path, config: &NerdcliConfig) -> io::Result<()> {
    if metadata(&path)?.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {
                if config.include_folders.is_empty()
                    || config
                        .include_folders
                        .iter()
                        .any(|f| full_path.ends_with(f))
                {
                    _list_files(vec, &full_path, config)?
                } else {
                }
            } else {
                if let Some(ext) = full_path.extension().and_then(|ext| ext.to_str()) {
                    if ext.to_string() == "toml" {
                        vec.push(full_path);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn list_files(path: &Path, config: &NerdcliConfig) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    _list_files(&mut vec, &path, config)?;
    Ok(vec)
}

pub fn get_quote(path: &Path, config: &NerdcliConfig) -> Quote {
    let default_quote = Quote {
        text: "The only way to do great work is to love what you do.".to_string(),
        author: "Steve Jobs".to_string(),
        source: None,
        date: None,
    };

    let mut rng = rand::rng();
    let binding = "en".to_string();
    let selected_language = config.quote_languages.choose(&mut rng).unwrap_or(&binding);

    let quote_files = list_files(&path.join(selected_language), config).unwrap_or_default();

    if quote_files.is_empty() {
        println!(
            "No quote files found for the selected language: {}",
            selected_language
        );
        return default_quote;
    }

    let selected_file = quote_files.choose(&mut rng).unwrap();

    let quotes: Vec<Quote> = parse_quotes(selected_file.to_str().unwrap()).quotes;
    if quotes.is_empty() {
        println!("No quotes found in the file: {}", selected_file.display());
        return default_quote;
    }

    let sq = quotes.choose(&mut rng);
    if sq.is_none() {
        println!("No quotes found in the file: {}", selected_file.display());
        return default_quote;
    } else {
        return sq.unwrap().clone();
    }
}

pub fn parse_quotes(filename: &str) -> Quotes {
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read quote file `{}`", filename);
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };
    let data: Quotes = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load quote data from `{}`: {}", filename, e);
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };

    data
}
