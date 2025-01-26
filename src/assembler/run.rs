use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::assembler::parser::parse_asm_file;
use crate::assembler::symbol_table::create_symbol_table;

pub fn run_assembler<P>(file: P, debug: bool)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Running assembler on {:?}", file);
    if !check_filetype(&file) {
        return;
    }
    let symbol_table = create_symbol_table();

    let output_path = create_output_path(&file);

    let output = parse_asm_file(&file, symbol_table, debug);
    if debug {
        println!("Output is:\n{}", output);
    }

    let mut file = File::create(output_path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn check_filetype<P>(file: &P) -> bool
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let filetype = Path::new(file.as_ref()).extension();
    match filetype {
        Some(extension) => {
            if extension != "asm" {
                println!("Path supplied isn't an .asm file");
                return false;
            }
            true
        }
        None => {
            println!("Path supplied isn't an .asm file");
            false
        }
    }
}

fn create_output_path<P>(file: P) -> PathBuf
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut output_file = PathBuf::from(file.as_ref());
    output_file.set_extension("hack");
    let output_file = output_file
        .file_name()
        .expect("Already checked from file_type");

    println!("Saving outputs to {:?}", output_file);
    PathBuf::from(output_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn run_assm_add() {
    //     run_assembler("../nand2tetris/nand2tetris/projects/6/add/Add.asm");
    // }

    // #[test]
    // fn run_assm_max() {
    //     run_assembler("../nand2tetris/nand2tetris/projects/6/max/Max.asm", false);
    // }

    // #[test]
    // fn run_assm_rect() {
    //     run_assembler("../nand2tetris/nand2tetris/projects/6/rect/Rect.asm", true);
    // }

    // #[test]
    // fn run_assm_pong() {
    //     run_assembler("../nand2tetris/nand2tetris/projects/6/pong/Pong.asm", true);
    // }
}
