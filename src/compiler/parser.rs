use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufRead};
use std::path::PathBuf;

use crate::compiler::keywords::make_keywords_array;
use crate::compiler::symbol_table::SymbolTable;
use crate::compiler::symbols::{funky_symbols, make_symbols_array};
use crate::compiler::tokens::Token;

#[derive(Clone)]
pub struct Compiler {
    pub file_path: PathBuf,
    pub debug: bool,
    pub tokens: Vec<Token>,
    pub multi_line_comment: bool,
    pub symbols_list: [String; 23],
    pub funky_symbols: HashMap<String, String>,
    pub keywords_list: [String; 21],
    pub output: String,
    pub output_padding: usize,
    pub code: String,
    pub class_type: String,
    pub class_symbol_table: SymbolTable,
    pub subroutine_symbol_table: SymbolTable,
}

impl Compiler {
    pub fn new(file_path: PathBuf, debug: bool) -> Self {
        Compiler {
            file_path,
            debug,
            tokens: vec![],
            multi_line_comment: false,
            symbols_list: make_symbols_array(),
            funky_symbols: funky_symbols(),
            keywords_list: make_keywords_array(),
            output: String::new(),
            output_padding: 0,
            code: String::new(),
            class_type: String::new(),
            class_symbol_table: SymbolTable::new(),
            subroutine_symbol_table: SymbolTable::new(),
        }
    }

    pub fn save_tokens(&mut self) {
        let output_path = self.create_output_path("Test");
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

    pub fn save_grammar_output(&mut self) {
        let output_path = self.create_output_path("Grammar");
        let mut output_file = File::create(output_path).unwrap();

        output_file.write_all(self.output.as_bytes()).unwrap();
    }

    fn create_output_path(&mut self, suffix: &str) -> PathBuf {
        let mut output_file = self.file_path.clone();
        let file_stem = output_file.file_stem().unwrap();
        let asm_file = format!("{}{}.xml", file_stem.to_str().unwrap(), suffix);
        output_file.pop();
        output_file.push(asm_file);

        output_file
    }

    pub fn compile_file(&mut self) {
        // read in text
        // break out into tokens
        self.tokenize_file();
        self.parse_tokens_to_grammar();
        self.save_tokens();
        self.save_grammar_output();
    }

    fn tokenize_file(&mut self) {
        let input_contents =
            File::open(&self.file_path).expect("At this point we should know we have a .jack file");

        // first round of tokenizing
        for line in io::BufReader::new(input_contents)
            .lines()
            .map_while(Result::ok)
        {
            self.tokenize(&line);
        }
    }
}
