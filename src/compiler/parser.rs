use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufRead};
use std::path::PathBuf;

use crate::compiler::keywords::make_keywords_array;
use crate::compiler::symbols::{funky_symbols, make_symbols_array};
use crate::compiler::tokens::Token;

pub struct Compiler {
    file_path: PathBuf,
    pub debug: bool,
    output: String,
    pub tokens: Vec<Token>,
    pub multi_line_comment: bool,
    pub symbols_list: [String; 23],
    pub funky_symbols: HashMap<String, String>,
    pub keywords_list: [String; 21],
}

impl Compiler {
    pub fn new(file_path: PathBuf, debug: bool) -> Self {
        Compiler {
            file_path,
            debug,
            output: String::new(),
            tokens: vec![],
            multi_line_comment: false,
            symbols_list: make_symbols_array(),
            funky_symbols: funky_symbols(),
            keywords_list: make_keywords_array(),
        }
    }

    pub fn save_to_vm(&mut self) {
        let output_path = self.create_output_path();
        let mut output_file = File::create(output_path).unwrap();

        output_file
            .write_all(String::from("<tokens>\n").as_bytes())
            .unwrap();
        for token in &self.tokens {
            let token_string = format!("{}\n", token);
            // let token_bytes = token.to_string().as_bytes();
            output_file.write_all(token_string.as_bytes()).unwrap();
        }
        output_file
            .write_all(String::from("</tokens>\n").as_bytes())
            .unwrap();
        // output_file.write_all(self.output.as_bytes()).unwrap();
    }

    fn create_output_path(&mut self) -> PathBuf {
        let mut output_file = self.file_path.clone();
        let file_stem = output_file.file_stem().unwrap();
        let asm_file = format!("{}Test.xml", file_stem.to_str().unwrap());
        output_file.pop();
        output_file.push(asm_file);

        output_file
    }

    pub fn compile_file(&mut self) {
        // read in text
        // break out into tokens
        self.tokenize_file();
        if self.debug {
            println!("{:?}", self.tokens);
        }
        self.parse_tokens_to_grammar();
        self.save_to_vm();
    }

    fn tokenize_file(&mut self) {
        let input_contents =
            File::open(&self.file_path).expect("At this point we should know we have a .jack file");

        // first round of tokenizing
        for line in io::BufReader::new(input_contents)
            .lines()
            .map_while(Result::ok)
        {
            if self.debug {
                println!("{}", line);
            }
            self.tokenize(&line);
        }
    }
}
