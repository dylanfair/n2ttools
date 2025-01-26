use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use crate::vm::arithmetic::handle_arithmetic;
use crate::vm::commands::CommandType;

pub fn parse_vm_file<P>(file: P, debug: bool) -> String
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if debug {
        println!("Parsing {}", file.as_ref().display());
    }
    let input_contents = File::open(file).expect("At this point we should know we have a .vm file");

    let stack_pointer = 256;
    let mut output = set_up_stack(stack_pointer);

    for line in io::BufReader::new(input_contents)
        .lines()
        .map_while(Result::ok)
    {
        if debug {
            println!("{}", line);
        }
        let parsed_output = parse_line(line);
        if debug {
            println!("Stack pointer is now at {}", stack_pointer);
            println!("{}", parsed_output);
        }
        output += &parsed_output;
    }

    output
}

fn parse_line(line: String) -> String {
    let trimmed_line = line.trim();

    if trimmed_line.starts_with("//") | trimmed_line.is_empty() {
        return String::from("");
    }

    let tokens: Vec<&str> = trimmed_line.split(" ").collect();
    let command = CommandType::from_str(tokens[0]).expect("Line isn't empty now");

    match command {
        CommandType::Push => handle_push(tokens),
        CommandType::Pop => handle_pop(tokens),
        CommandType::Arithmetic => handle_arithmetic(tokens),
        _ => String::from("todo"),
    }
}

fn set_up_stack(stack_pointer: u32) -> String {
    format!("@{}\nD=A\n@SP\nM=D\n", stack_pointer)
    // String::from("")
}

fn handle_pop(tokens: Vec<&str>) -> String {
    // LCL = RAM[SP--]
    // @SP
    // A=M
    // D=M
    // @SP
    // M=M-1
    // @LCL
    // A=M
    // M=D
    // @LCL
    // M=M+1
    let segment = tokens[1];
    let value = tokens[2];
    todo!();
}

fn handle_push(tokens: Vec<&str>) -> String {
    // RAM[SP++] = D
    // @SP
    // A=M
    // M=D
    // @SP
    // M=M+1
    //
    // RAM[SP++] = number
    // @number
    // D=A
    // @SP
    // A=M
    // M=D
    // @SP
    // M=M+1
    let segment = tokens[1];
    let value = tokens[2];

    let location = segment_to_location(segment);

    let mut output = format!("@{}\n", value);
    output += "D=A\n";
    output += &format!("@{}\n", location);
    output += "A=M\n";
    output += "M=D\n";
    output += &format!("@{}\n", location);
    output += "M=M+1\n";
    output
}

fn segment_to_location(segment: &str) -> &str {
    match segment {
        "constant" => "SP",
        "local" => "LCL",
        "argument" => "ARG",
        "this" => "THIS",
        "that" => "THAT",
        "temp" => "TEMP",
        _ => panic!(),
    }
}
