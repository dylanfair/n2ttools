use std::path::{Path, PathBuf};

use crate::vm::parser::parse_vm_file;

pub fn run_vm<P>(file: P, debug: bool)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Running the vm on '{}'", file.as_ref().display());
    if !check_filetype(&file) {
        return;
    }
    let output_path = create_output_path(&file);

    let output = parse_vm_file(&file, debug);
}

fn check_filetype<P>(file: &P) -> bool
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let filetype = Path::new(file.as_ref()).extension();
    match filetype {
        Some(extension) => {
            if extension != "vm" {
                println!("Path supplied isn't an .vm file");
                return false;
            }
            true
        }
        None => {
            println!("Path supplied isn't an .vm file");
            false
        }
    }
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
            "../nand2tetris/nand2tetris/projects/7/StackArithmetic/SimpleAdd.vm",
            true,
        );
    }
}
