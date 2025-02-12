use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use crate::vm::commands::CommandType;

pub fn set_up_stack() -> String {
    let mut stack = String::from("@256\nD=A\n@SP\nM=D\n");
    stack += "@300\nD=A\n@LCL\nM=D\n";
    stack += "@400\nD=A\n@ARG\nM=D\n";
    stack += "@3000\nD=A\n@THIS\nM=D\n";
    stack += "@3010\nD=A\n@THAT\nM=D\n";
    stack
}

pub struct Parser {
    pub output: String,
    pub general_return_number: u64,
    pub caller_return_number: u64,
    temp_base: u32,
    pub file_name: String,
    pub function_name: String,
}

impl Parser {
    fn new(temp_base: u32, file_name: String) -> Self {
        Parser {
            general_return_number: 0,
            caller_return_number: 0,
            output: String::new(),
            temp_base,
            file_name,
            function_name: String::new(),
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
            CommandType::Label => self.handle_label(tokens),
            CommandType::If => self.handle_if_goto(tokens),
            CommandType::Goto => self.handle_goto(tokens),
            CommandType::Function => self.handle_function(tokens),
            CommandType::Return => self.handle_return(),
            CommandType::Call => self.handle_call(tokens),
        }
    }

    pub fn pop_stack(&mut self) {
        // first grab the top stack value
        self.output += "@SP\n";
        self.output += "M=M-1\n";
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "D=M\n"; // now stored in D
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

        if segment == "pointer" {
            self.pop_stack();

            if value == "0" {
                self.output += "@THIS\n";
            } else {
                self.output += "@THAT\n";
            };
            self.output += "M=D\n"; // push popped value to pointer
        } else if segment == "this" {
            self.output += "@THIS\n";
            self.output += "D=M\n";
            self.output += &format!("@{}\n", value);
            self.output += "D=D+A\n";
            self.output += "@R15\n";
            self.output += "M=D\n";

            self.pop_stack();

            self.output += "@R15\n";
            self.output += "A=M\n";
            self.output += "M=D\n";
        } else if segment == "that" {
            self.output += "@THAT\n";
            self.output += "D=M\n";
            self.output += &format!("@{}\n", value);
            self.output += "D=D+A\n";
            self.output += "@R15\n";
            self.output += "M=D\n";

            self.pop_stack();

            self.output += "@R15\n";
            self.output += "A=M\n";
            self.output += "M=D\n";
        } else if segment == "local" {
            self.output += "@LCL\n";
            self.output += "D=M\n";
            self.output += &format!("@{}\n", value);
            self.output += "D=D+A\n";
            self.output += "@R15\n";
            self.output += "M=D\n";

            self.pop_stack();

            self.output += "@R15\n";
            self.output += "A=M\n";
            self.output += "M=D\n";
        } else if segment == "argument" {
            self.output += "@ARG\n";
            self.output += "D=M\n";
            self.output += &format!("@{}\n", value);
            self.output += "D=D+A\n";
            self.output += "@R15\n";
            self.output += "M=D\n";

            self.pop_stack();

            self.output += "@R15\n";
            self.output += "A=M\n";
            self.output += "M=D\n";
        } else if segment == "static" {
            self.pop_stack();

            self.output += &format!("@{}.{}\n", self.file_name, value);
            self.output += "M=D\n";
        } else if segment == "temp" {
            self.pop_stack();

            self.output += &format!("@{}\n", self.temp_base + value.parse::<u32>().unwrap());
            self.output += "M=D\n";
        } else {
            panic!("Found an unknown segment: {}", segment);
        };
    }

    pub fn push_d(&mut self) {
        // Then push to stack
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "M=D\n";
        self.output += "@SP\n";
        self.output += "M=M+1\n";
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

        match segment {
            "constant" => {
                self.output += &format!("@{}\n", value);
                self.output += "D=A\n";
            }
            "pointer" => {
                if value == "0" {
                    self.output += "@THIS\n";
                } else {
                    self.output += "@THAT\n";
                };
                self.output += "D=M\n"; // now stored in D
            }
            "this" => {
                self.output += "@THIS\n";
                self.output += "D=M\n";
                self.output += &format!("@{}\n", value);
                self.output += "D=D+A\n";
                self.output += "A=D\n";
                self.output += "D=M\n";
            }
            "that" => {
                self.output += "@THAT\n";
                self.output += "D=M\n";
                self.output += &format!("@{}\n", value);
                self.output += "D=D+A\n";
                self.output += "A=D\n";
                self.output += "D=M\n";
            }
            "local" => {
                self.output += "@LCL\n";
                self.output += "D=M\n";
                self.output += &format!("@{}\n", value);
                self.output += "D=D+A\n";
                self.output += "A=D\n";
                self.output += "D=M\n";
            }
            "argument" => {
                self.output += "@ARG\n";
                self.output += "D=M\n";
                self.output += &format!("@{}\n", value);
                self.output += "D=D+A\n";
                self.output += "A=D\n";
                self.output += "D=M\n";
            }
            "static" => {
                self.output += &format!("@{}.{}\n", self.file_name, value);
                self.output += "D=M\n";
            }
            "temp" => {
                let index = self.temp_base + value.parse::<u32>().unwrap();
                self.output += &format!("@{}\n", index);
                self.output += "D=M\n"; // now stored in D
            }
            _ => {
                panic!("Found an unknown segment: {}", segment);
            }
        }
        self.push_d();
    }
}

pub fn parse_vm_file<P>(file: P, debug: bool) -> String
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if debug {
        println!("Parsing {}", file.as_ref().display());
    }
    let file_name = file
        .as_ref()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let input_contents = File::open(file).expect("At this point we should know we have a .vm file");

    let mut parser = Parser::new(5, file_name);

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
