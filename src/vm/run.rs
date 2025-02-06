use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::vm::parser::parse_vm_file;

pub fn run_vm<P>(path: P, debug: bool)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if !check_for_valid_files(&path) {
        return;
    }
    println!("Running the vm on '{}'", path.as_ref().display());
    let output_path = create_output_path(&path);

    let output = parse_vm_file(&path, debug);
    if debug {
        println!("Output is:\n{}", output);
    }

    let mut file = File::create(output_path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn check_for_valid_files<P>(file: &P) -> bool
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let file_path = Path::new(file.as_ref());
    if file_path == Path::new(".") {
        println!("Looking for .vm files in the current folder");
    }

    if file_path.is_dir() {
        println!(
            "Checking for .vm files in the following directory: {}",
            file_path.display()
        );
    }

    if file_path.is_file() {
        let filetype = file_path.extension();
        match filetype {
            Some(extension) => {
                if extension != "vm" {
                    println!("Path supplied isn't an .vm file");
                    return false;
                }
                return true;
            }
            None => {
                println!("Path supplied isn't an .vm file");
                return false;
            }
        }
    }

    println!("Could not find a valid file");
    false
}

fn create_output_path<P>(file: P) -> PathBuf
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut output_file = PathBuf::from(file.as_ref());
    output_file.set_extension("asm");
    let output_file = output_file
        .file_name()
        .expect("Already checked from file_type");

    println!("Saving outputs to {:?}", output_file);
    PathBuf::from(output_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_vm() {
        run_vm(
            "../nand2tetris/nand2tetris/projects/7/StackArithmetic/SimpleAdd/SimpleAdd.vm",
            true,
        );
    }
}
