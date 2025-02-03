use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use crate::vm::commands::CommandType;

pub struct Parser {
    pub output: String,
    pub return_caller_number: u64,
    local_base: u32,
    arg_base: u32,
    this_base: u32,
    that_base: u32,
    temp_base: u32,
}

fn set_up_stack(
    stack_pointer: u32,
    local_base: u32,
    arg_base: u32,
    this_base: u32,
    that_base: u32,
) -> String {
    let mut stack = format!("@{}\nD=A\n@SP\nM=D\n", stack_pointer);
    stack += &format!("@{}\nD=A\n@LCL\nM=D\n", local_base);
    stack += &format!("@{}\nD=A\n@ARG\nM=D\n", arg_base);
    stack += &format!("@{}\nD=A\n@THIS\nM=D\n", this_base);
    stack += &format!("@{}\nD=A\n@THAT\nM=D\n", that_base);
    stack
}

impl Parser {
    fn new(
        stack_pointer: u32,
        local_base: u32,
        arg_base: u32,
        this_base: u32,
        that_base: u32,
        temp_base: u32,
    ) -> Self {
        Parser {
            return_caller_number: 0,
            output: set_up_stack(stack_pointer, local_base, arg_base, this_base, that_base),
            local_base,
            arg_base,
            this_base,
            that_base,
            temp_base,
        }
    }

    fn parse_line(&mut self, line: String) {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("//") | trimmed_line.is_empty() {
            return;
        }

        let tokens: Vec<&str> = trimmed_line.split(" ").collect();
        let command = CommandType::from_str(tokens[0]).expect("Line isn't empty now");

        match command {
            CommandType::Push => self.handle_push(tokens),
            CommandType::Pop => self.handle_pop(tokens),
            CommandType::Arithmetic => self.handle_arithmetic(tokens),
            _ => (),
        }
    }

    fn handle_pop(&mut self, tokens: Vec<&str>) {
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

        let location = segment_to_location(segment);

        // first grab the top stack value
        self.output += "@SP\n";
        self.output += "M=M-1\n";
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "D=M\n"; // now stored in D

        let segment_base = self.get_segment_base(&location);
        let index = segment_base + value.parse::<u32>().unwrap();
        self.output += &format!("@{}\n", index);
        self.output += "M=D\n";
    }

    fn handle_push(&mut self, tokens: Vec<&str>) {
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

        if location == "SP" {
            self.output += &format!("@{}\n", value);
            self.output += "D=A\n";
        } else {
            // first grab the segment value
            let segment_base = self.get_segment_base(&location);
            let index = segment_base + value.parse::<u32>().unwrap();
            self.output += &format!("@{}\n", index);
            self.output += "D=M\n"; // now stored in D
        }
        // Then push to stack
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "M=D\n";
        self.output += "@SP\n";
        self.output += "M=M+1\n";
    }

    fn get_segment_base(&mut self, location: &str) -> u32 {
        match location {
            "SP" => panic!("Shouldn't see SP here"),
            "LCL" => self.local_base,
            "ARG" => self.arg_base,
            "THIS" => self.this_base,
            "THAT" => self.that_base,
            "TEMP" => self.temp_base,
            _ => panic!("Found an unknown location: {}", location),
        }
    }
}

fn segment_to_location(segment: &str) -> String {
    match segment {
        "constant" => "SP".to_string(),
        "local" => "LCL".to_string(),
        "argument" => "ARG".to_string(),
        "this" => "THIS".to_string(),
        "that" => "THAT".to_string(),
        "temp" => "TEMP".to_string(),
        _ => panic!("Found an unknown segment: {}", segment),
    }
}

pub fn parse_vm_file<P>(file: P, debug: bool) -> String
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if debug {
        println!("Parsing {}", file.as_ref().display());
    }
    let input_contents = File::open(file).expect("At this point we should know we have a .vm file");

    let mut parser = Parser::new(256, 300, 400, 3000, 3010, 5);

    for line in io::BufReader::new(input_contents)
        .lines()
        .map_while(Result::ok)
    {
        if debug {
            println!("{}", line);
        }
        parser.parse_line(line);
    }

    parser.output
}
