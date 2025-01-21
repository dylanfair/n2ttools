use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::assembler::symbol_table::{self, create_symbol_table};

pub fn run_assembler<P>(file: P)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Running assembler on {:?}", file);
    if !check_filetype(&file) {
        return;
    }
    let symbol_table = create_symbol_table();

    let output_path = create_output_path(&file);
    let output = parse_asm_file(&file, symbol_table);
}

fn parse_asm_file<P>(file: &P, mut symbol_table: BTreeMap<String, u32>)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("{:?}", file);
    let input_contents =
        File::open(file).expect("At this point we should know we have a .asm file");

    let mut current_line = 0;
    let mut free_symbols_pointer = 16;
    for line in io::BufReader::new(input_contents)
        .lines()
        .map_while(Result::ok)
    {
        // Parse lines here
        println!("{}", line);
        (current_line, free_symbols_pointer) =
            parse_line(line, &mut symbol_table, current_line, free_symbols_pointer);
        println!("{}", current_line);
    }

    println!("{:?}", symbol_table);
}

fn parse_line(
    line: String,
    symbol_table: &mut BTreeMap<String, u32>,
    current_line: u64,
    mut free_symbols_pointer: u32,
) -> (u64, u32) {
    // First check for moments we skip
    if line.starts_with("//") | line.is_empty() {
        return (current_line, free_symbols_pointer);
    }

    // Now parse for symbols
    let cleaned_line = line.trim().to_string();
    if cleaned_line.starts_with("@") {
        let cleaned_value = cleaned_line.replace("@", "");
        // if a number, just move on as its an A-instruction
        let mut symbol_chars = cleaned_value.chars();
        let first_char = symbol_chars.next().expect("wouldn't be blank");
        if first_char.is_numeric() {
            return (current_line + 1, free_symbols_pointer);
        }

        // if not a number, we insert to symbol table with a value
        symbol_table
            .entry(cleaned_value)
            .or_insert(free_symbols_pointer);
        free_symbols_pointer += 1
    }

    // label declarations
    if cleaned_line.starts_with("(") & !symbol_table.contains_key(&cleaned_line) {
        symbol_table.insert(cleaned_line, current_line as u32 + 1);
    }
    (current_line + 1, free_symbols_pointer)
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

    #[test]
    fn run_assm_add() {
        run_assembler("../nand2tetris/nand2tetris/projects/6/add/Add.asm");
    }

    #[test]
    fn run_assm_() {
        run_assembler("../nand2tetris/nand2tetris/projects/6/max/Max.asm");
    }
}
