use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::vm::parser::{parse_vm_file, set_up_stack};

pub fn run_vm<P>(path: P, debug: bool)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Running the vm on '{}'", path.as_ref().display());

    let files = valid_files(&path);
    if files.is_none() {
        println!("Could not find any valid '.vm' files to work on.");
        return;
    }

    // let mut output = set_up_stack();
    let mut output = String::new();
    for file in files.expect("Should have something after .is_none() check") {
        let parsed_output = parse_vm_file(file, debug);
        if debug {
            println!("Output is:\n{}", parsed_output);
        }
        output += &parsed_output;
    }

    let output_path = create_output_path(&path);
    let mut file = File::create(output_path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn valid_files<P>(file: &P) -> Option<Vec<PathBuf>>
where
    P: AsRef<Path> + std::fmt::Debug + ?Sized,
{
    let file_path = Path::new(file.as_ref());
    if !file_path.exists() {
        println!("Path provided does not exist");
        return None;
    }

    if file_path.is_dir() {
        let mut files = vec![];
        for entry in file_path
            .read_dir()
            .expect("We checked if this is a directory")
            .flatten()
        {
            let entry_path = entry.path();
            let extension = entry_path.extension();
            if let Some(extension) = extension {
                if extension == "vm" {
                    files.push(entry.path().to_path_buf())
                }
            }
        }

        if files.is_empty() {
            return None;
        }
        return Some(files);
    }

    if file_path.is_file() {
        let filetype = file_path.extension();
        match filetype {
            Some(extension) => {
                if extension != "vm" {
                    println!("Path supplied isn't an .vm file");
                    return None;
                }
                return Some(vec![file_path.to_path_buf()]);
            }
            None => {
                println!("Path supplied isn't an .vm file");
                return None;
            }
        }
    }

    println!("Could not find a valid file");
    None
}

fn create_output_path<P>(file: P) -> PathBuf
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut input_file = PathBuf::from(file.as_ref());

    let output_file = if input_file.is_dir() {
        if input_file == PathBuf::from(".") {
            let mut current_dir = env::current_dir().expect("Should be in some directory");
            let parent_name = current_dir.file_name().unwrap().to_str().unwrap();
            current_dir.push(format!("{}.asm", parent_name));
            current_dir
        } else {
            let parent_name = input_file.file_name().unwrap().to_str().unwrap();
            input_file.push(format!("{}.asm", parent_name));
            input_file
        }
    } else {
        input_file.set_extension("asm");
        input_file
    };

    output_file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filepaths() {
        let directory = valid_files("../nand2tetris/nand2tetris/projects/8/ProgramFlow/BasicLoop");
        assert_eq!(
            directory.unwrap(),
            vec![Path::new(
                "../nand2tetris/nand2tetris/projects/8/ProgramFlow/BasicLoop/BasicLoop.vm"
            )]
        );

        let single_file =
            valid_files("../nand2tetris/nand2tetris/projects/8/ProgramFlow/BasicLoop/BasicLoop.vm");
        assert_eq!(
            single_file.unwrap(),
            vec![Path::new(
                "../nand2tetris/nand2tetris/projects/8/ProgramFlow/BasicLoop/BasicLoop.vm"
            )
            .to_path_buf()]
        );

        let multiple_files =
            valid_files("../nand2tetris/nand2tetris/projects/8/FunctionCalls/StaticsTest");
        assert!(multiple_files.as_ref().unwrap().contains(
            &Path::new("../nand2tetris/nand2tetris/projects/8/FunctionCalls/StaticsTest/Class1.vm")
                .to_path_buf()
        ));
        assert!(multiple_files.as_ref().unwrap().contains(
            &Path::new("../nand2tetris/nand2tetris/projects/8/FunctionCalls/StaticsTest/Class2.vm")
                .to_path_buf()
        ));
        assert!(multiple_files.as_ref().unwrap().contains(
            &Path::new("../nand2tetris/nand2tetris/projects/8/FunctionCalls/StaticsTest/Sys.vm")
                .to_path_buf()
        ));
    }

    #[test]
    fn test_parse_current_directory() {
        let current_directory = valid_files(".");
        assert!(current_directory.is_none());
    }

    // #[test]
    // fn test_run_vm() {
    //     run_vm(
    //         "../nand2tetris/nand2tetris/projects/7/StackArithmetic/SimpleAdd/SimpleAdd.vm",
    //         true,
    //     );
    // }
}
