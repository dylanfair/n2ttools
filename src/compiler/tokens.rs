use std::fmt::Display;

use crate::compiler::parser::Compiler;

#[derive(Debug)]
enum TokenType {
    Keyword,
    Symbol,
    Identifier,
}

#[derive(Debug)]
pub struct Token {
    token_str: String,
    token_type: TokenType,
}

impl Token {
    fn new(token_str: String, token_type: TokenType) -> Self {
        Token {
            token_str,
            token_type,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::Symbol => write!(f, "<symbol> {} </symbol>", self.token_str),
            TokenType::Keyword => write!(f, "<keyword> {} </keyword>", self.token_str),
            TokenType::Identifier => write!(f, "<identifier> {} </identifier>", self.token_str),
        }
    }
}

impl Compiler {
    fn skip_line(&mut self, text_line: &str) -> bool {
        if text_line.starts_with("//") | text_line.is_empty() {
            return true;
        }
        if text_line.starts_with("/*") {
            self.multi_line_comment = true;
        }
        if self.multi_line_comment & text_line.ends_with("*/") {
            self.multi_line_comment = false;
            return true;
        }
        if self.multi_line_comment {
            return true;
        }

        false
    }

    fn make_token(&mut self, token_str: String) -> Token {
        if self.symbols_list.contains(&token_str) {
            return Token::new(token_str, TokenType::Symbol);
        }
        if self.keywords_list.contains(&token_str) {
            return Token::new(token_str, TokenType::Keyword);
        }

        Token::new(token_str, TokenType::Identifier)
    }

    pub fn tokenize(&mut self, text_line: &str) {
        let trimmed_line = text_line.trim();
        if self.skip_line(trimmed_line) {
            return;
        }

        // remove inline comments
        if self.debug {
            println!("{}", trimmed_line);
        }
        let ready_line = if trimmed_line.contains("//") {
            let (cleaned_line, _) = trimmed_line.split_once("//").unwrap();
            cleaned_line
        } else {
            trimmed_line
        };

        let mut token = String::new();
        for char in ready_line.chars() {
            // check for whitespaces
            // push existing token if we have one
            // assign expression booleans if relevant
            if char.is_whitespace() & !token.is_empty() {
                let token_type = self.make_token(token);
                self.tokens.push(token_type);
                token = String::new();
            }
            if char.is_whitespace() {
                continue;
            }

            // deal with general symbols
            if self.symbols_list.contains(&char.to_string()) {
                if !token.is_empty() {
                    let token_type = self.make_token(token);
                    self.tokens.push(token_type);
                    token = String::new();
                }
                let token_type = self.make_token(char.to_string());
                self.tokens.push(token_type);
                continue;
            }

            // now push to char
            token.push(char);
        }
        // if we reached the end of the line, push in recent token
        if !token.is_empty() {
            let token_type = self.make_token(token);
            self.tokens.push(token_type);
        }
    }
}
