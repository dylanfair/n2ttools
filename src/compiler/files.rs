use std::path::{Path, PathBuf};

/// Returns paths to valid .jack files
pub fn valid_files(file: &Path) -> Option<Vec<PathBuf>> {
    if !file.exists() {
        println!("Path provided does not exist");
        return None;
    }

    if file.is_dir() {
        let mut files = vec![];
        for entry in file
            .read_dir()
            .expect("We checked if this is a directory")
            .flatten()
        {
            let entry_path = entry.path();
            let extension = entry_path.extension();
            if let Some(extension) = extension {
                if extension == "jack" {
                    files.push(entry.path().to_path_buf())
                }
            }
        }

        if files.is_empty() {
            return None;
        }
        return Some(files);
    }

    if file.is_file() {
        let filetype = file.extension();
        match filetype {
            Some(extension) => {
                if extension != "jack" {
                    println!("Path supplied isn't an .jack file");
                    return None;
                }
                return Some(vec![file.to_path_buf()]);
            }
            None => {
                println!("Path supplied isn't an .jack file");
                return None;
            }
        }
    }

    println!("Could not find a valid file");
    None
}
