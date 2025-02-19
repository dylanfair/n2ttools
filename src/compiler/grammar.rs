use crate::compiler::parser::Compiler;

use super::tokens::{Token, TokenType};

impl Compiler {}

pub enum Statements {
    Let,
    If,
    While,
    Do,
    Return,
}

impl Compiler {
    pub fn parse_tokens_to_grammar(self) {
        let tokens_iter = self.tokens.iter();

        self.process_next(tokens_iter, "class".to_string(), TokenType::Keyword);
    }

    fn process_next<'a>(
        self,
        mut tokens_iter: impl Iterator<Item = &'a Token>,
        expected_token_str: String,
        expected_token_type: TokenType,
    ) {
        let next_token = tokens_iter.next().unwrap();
        let expected_token = Token::new(expected_token_str, expected_token_type);

        if *next_token != expected_token {
            panic!(
                "The first token in a file should be a '{}'. Found '{}'",
                expected_token, next_token
            )
        }
    }
}
