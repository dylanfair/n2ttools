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

fn second_pass_parse_line(line: &str, symbol_table: &mut BTreeMap<String, u32>) -> String {
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

fn parse_a_instruction(instruction: String) -> String {
    // 15 bit value of instruction
    // given a number, convert to bits then pad with 0s?
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

    let c_instruction = format!("111{comp_b}{dest_b}{jump_b}");

    return c_instruction;
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

    return answer.to_string();
}

fn dest_binary(dest: &str) -> String {
    let answer = match dest {
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => {
            panic!()
        }
    };

    return answer.to_string();
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

    return answer.to_string();
}
