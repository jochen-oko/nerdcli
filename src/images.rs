
use std::{fs::{self, metadata}, io, path::{Path, PathBuf}};

use crate::settings::NerdcliConfig;

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path, config: &NerdcliConfig) -> io::Result<()> {
    if metadata(&path)?.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {

                if config.include_folders.is_empty() 
                  || config.include_folders.iter().any(
                    |f|full_path.ends_with(f)
                ) {
                    // println!("reading dir {}", full_path.display());
                    _list_files(vec, &full_path, config)?
                } else {
                    // println!("not include {}", full_path.display());
                }
            } else {
                if let Some(ext) = full_path.extension().and_then(|ext| ext.to_str()) {
                    if config.image_types.is_empty() || config.image_types.contains(&ext.to_string()) {
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