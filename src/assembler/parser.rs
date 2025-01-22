use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_asm_file<P>(file: &P, mut symbol_table: BTreeMap<String, u32>, debug: bool) -> String
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let input_contents =
        File::open(file).expect("At this point we should know we have a .asm file");

    let mut second_pass = input_contents.try_clone().expect("Hopefully this works?");
    let mut current_line = 0;

    // Parse lines here for symbol table
    for line in io::BufReader::new(input_contents)
        .lines()
        .map_while(Result::ok)
    {
        if debug {
            println!("{} {}", current_line, line);
        }
        current_line = first_pass_parse_line(&line, &mut symbol_table, current_line);
    }
    if debug {
        println!("{:?}", symbol_table);
    }

    // now parse for translations using symbol table
    let mut output = String::new();
    #[allow(unused_assignments)]
    let mut parse_output = String::new();
    let mut free_symbols_pointer = 16;

    second_pass.rewind().unwrap();
    for line in io::BufReader::new(second_pass)
        .lines()
        .map_while(Result::ok)
    {
        (parse_output, free_symbols_pointer) =
            second_pass_parse_line(&line, &mut symbol_table, free_symbols_pointer);

        output += &parse_output;
    }
    if debug {
        println!("{:?}", symbol_table);
    }

    output
}

fn first_pass_parse_line(
    line: &str,
    symbol_table: &mut BTreeMap<String, u32>,
    current_line: u64,
) -> u64 {
    // First check for moments we skip
    let cleaned_line = line.trim().to_string();
    if cleaned_line.starts_with("//") | cleaned_line.is_empty() {
        return current_line;
    }

    // label declarations
    if cleaned_line.starts_with("(") & !symbol_table.contains_key(&cleaned_line) {
        let mut insert = cleaned_line.strip_suffix(")").unwrap();
        insert = insert.strip_prefix("(").unwrap();
        symbol_table.insert(insert.to_string(), current_line as u32);
        return current_line;
    }
    current_line + 1
}

fn second_pass_parse_line(
    line: &str,
    symbol_table: &mut BTreeMap<String, u32>,
    mut free_symbols_pointer: u32,
) -> (String, u32) {
    // First check for moments we skip
    let mut output = String::from("");
    let cleaned_line = line.trim().to_string();
    if cleaned_line.starts_with("//") | cleaned_line.is_empty() | cleaned_line.starts_with("(") {
        return (output, free_symbols_pointer);
    }

    // Now parse for symbols
    if cleaned_line.starts_with("@") {
        (output, free_symbols_pointer) =
            parse_a_instruction(cleaned_line, symbol_table, free_symbols_pointer);
        (output, free_symbols_pointer)
    } else {
        output = parse_c_instruction(cleaned_line);
        (output, free_symbols_pointer)
    }
}

fn parse_a_instruction(
    instruction: String,
    symbol_table: &mut BTreeMap<String, u32>,
    mut free_symbols_pointer: u32,
) -> (String, u32) {
    // 15 bit value of instruction
    // given a number, convert to bits then pad with 0s?
    // Now parse for symbols
    let cleaned_line = instruction.trim().to_string();
    let cleaned_value = cleaned_line.replace("@", "");

    // if a number, just move on as its an A-instruction
    let mut symbol_chars = cleaned_value.chars();
    let first_char = symbol_chars.next().expect("wouldn't be blank");
    if first_char.is_numeric() {
        // transform number to binary and return
        let number = cleaned_value.parse::<u32>().unwrap();
        return (format!("{number:016b}\n").to_string(), free_symbols_pointer);
    }

    if !symbol_table.contains_key(&cleaned_value) {
        symbol_table.insert(cleaned_value.clone(), free_symbols_pointer);
        free_symbols_pointer += 1;
    }

    let symbol_value = symbol_table
        .get(&cleaned_value)
        .expect("After initial parse we should never not see our symbol");
    // transform number to bianry
    (
        format!("{symbol_value:016b}\n").to_string(),
        free_symbols_pointer,
    )
}

fn parse_c_instruction(instruction: String) -> String {
    // dest = comp ; jump

    // we have a dest
    let mut dest_b = String::from("000");
    let mut jump_b = String::from("000");
    let mut comp_b = String::from("0000000");
    if instruction.contains("=") {
        let (dest, comp) = instruction.split_once("=").unwrap();
        dest_b = dest_binary(dest.trim());
        comp_b = comp_binary(comp.trim());
    }
    if instruction.contains(";") {
        let (comp, jump) = instruction.split_once(";").unwrap();
        comp_b = comp_binary(comp.trim());
        jump_b = jump_binary(jump.trim());
    }

    let c_instruction = format!("111{comp_b}{dest_b}{jump_b}\n");

    c_instruction
}

fn comp_binary(comp: &str) -> String {
    let answer = match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => {
            panic!()
        }
    };

    answer.to_string()
}

fn dest_binary(dest: &str) -> String {
    let answer = match dest {
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => {
            panic!()
        }
    };

    answer.to_string()
}

fn jump_binary(jump: &str) -> String {
    let answer = match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => {
            panic!()
        }
    };

    answer.to_string()
}
