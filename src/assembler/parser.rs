use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_asm_file<P>(file: &P, mut symbol_table: BTreeMap<String, u32>)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("{:?}", file);
    let input_contents =
        File::open(file).expect("At this point we should know we have a .asm file");

    let second_pass = input_contents.try_clone().expect("Hopefully this works?");
    let mut current_line = 0;
    let mut free_symbols_pointer = 16;

    // Parse lines here for symbol table
    for line in io::BufReader::new(input_contents)
        .lines()
        .map_while(Result::ok)
    {
        (current_line, free_symbols_pointer) =
            first_pass_parse_line(&line, &mut symbol_table, current_line, free_symbols_pointer);
        println!("{} {}", current_line, line);
    }
    println!("{:?}", symbol_table);

    // now parse for translations using symbol table
    for line in io::BufReader::new(second_pass)
        .lines()
        .map_while(Result::ok)
    {
        (current_line, free_symbols_pointer) =
            parse_line(&line, &mut symbol_table, current_line, free_symbols_pointer);
        println!("{}", current_line);
    }
}

fn first_pass_parse_line(
    line: &str,
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
        return (current_line, free_symbols_pointer);
    }
    (current_line + 1, free_symbols_pointer)
}

fn second_pass_parse_line(line: &str) -> String {
    // First check for moments we skip
    if line.starts_with("//") | line.is_empty() {
        return String::from("");
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
        return (current_line, free_symbols_pointer);
    }
    (current_line + 1, free_symbols_pointer)
}
