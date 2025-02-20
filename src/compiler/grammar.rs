use std::iter::Peekable;

use crate::compiler::parser::Compiler;

use super::tokens::{Token, TokenType};

impl Compiler {
    pub fn save_to_output(&mut self, grammar_string: &str) {
        let mut spaces = String::new();
        let spaces_count = self.output_padding;
        for _ in 0..spaces_count {
            spaces += " ";
        }
        self.output += &format!("{}{}\n", spaces, grammar_string);
    }

    pub fn parse_tokens_to_grammar(&mut self) {
        let tokens = self.tokens.clone();
        let mut tokens_iter = tokens.iter().peekable();

        self.save_to_output("<class>");
        self.output_padding += 2;

        // class
        self.process_specific(&mut tokens_iter, "class".to_string(), TokenType::Keyword);
        // class name
        self.process_type(&mut tokens_iter, TokenType::Identifier);
        // {
        self.process_type(&mut tokens_iter, TokenType::Symbol);
        // class variable declarations
        self.process_class_variable_declarations(&mut tokens_iter);

        // subroutine declarations
        self.process_subroutine_declarations(&mut tokens_iter);

        // parameter list
        // subroutine body
        // variable declarations

        self.output_padding -= 2;
        self.save_to_output("</class>");
    }

    fn process_subroutine_declarations<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // if we see 'static' or 'field' then we process next 4 tokens, repeating
        // static or field
        // type
        // variable name
        // ;
        let mut peek = tokens_iter.peek().cloned();
        loop {
            match peek {
                Some(p) => {
                    if (p.token_str == "constructor")
                        | (p.token_str == "function")
                        | (p.token_str == "method")
                    {
                        self.save_to_output("<subroutineDec>");
                        self.output_padding += 2;

                        // process constructor, function or method
                        self.process_type(tokens_iter, TokenType::Keyword);
                        // the type associated with the function
                        self.process_next(tokens_iter);

                        // name of the function
                        self.process_type(tokens_iter, TokenType::Identifier);

                        // parameters
                        self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                        self.process_parameter_list(tokens_iter);
                        self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                        // subroutineBody
                        self.process_subroutine_body(tokens_iter);

                        self.output_padding -= 2;
                        self.save_to_output("</subroutineDec>");
                        peek = tokens_iter.peek().cloned();
                    } else {
                        return;
                    }
                }
                None => {
                    return;
                }
            }
        }
    }

    fn process_subroutine_body<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<subroutineBody>");
        self.output_padding += 2;

        self.process_specific(tokens_iter, String::from("{"), TokenType::Symbol);

        // first check for var declarations
        self.process_subroutine_variable_declarations(tokens_iter);

        // Then iterate through statements
        self.process_statements(tokens_iter);

        self.output_padding -= 2;
        self.save_to_output("</subroutineBody>");
    }

    fn process_subroutine_variable_declarations<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // if we see 'static' or 'field' then we process next 4 tokens, repeating
        // static or field
        // type
        // variable name
        // ;
        let mut peek = tokens_iter.peek().cloned();
        loop {
            match peek {
                Some(p) => {
                    if p.token_str == "var" {
                        self.save_to_output("<varDec>");
                        self.output_padding += 2;
                        // static or field
                        self.process_type(tokens_iter, TokenType::Keyword);
                        // variable type
                        self.process_next(tokens_iter);
                        self.process_class_variable_names(tokens_iter);

                        self.output_padding -= 2;
                        self.save_to_output("</varDec>");
                        peek = tokens_iter.peek().cloned();
                    } else {
                        return;
                    }
                }
                None => {
                    return;
                }
            }
        }
    }

    fn process_parameter_list<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<parameterList>");
        self.output_padding += 2;

        let parenthesis_close_peek = tokens_iter.peek().cloned().unwrap();
        if parenthesis_close_peek.token_str == ")" {
            self.output_padding -= 2;
            self.save_to_output("</parameterList>");
            return;
        }

        // we have parameters! let's get them
        loop {
            // parameter type
            self.process_next(tokens_iter);
            // paramter name
            self.process_type(tokens_iter, TokenType::Identifier);
            // look for comma
            let comma_peek = tokens_iter.peek().cloned().unwrap();
            if comma_peek.token_str == "," {
                self.process_specific(tokens_iter, String::from(","), TokenType::Symbol);
            } else {
                break;
            }
        }

        self.output_padding -= 2;
        self.save_to_output("</parameterList>");
    }

    fn process_class_variable_declarations<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // if we see 'static' or 'field' then we process next 4 tokens, repeating
        // static or field
        // type
        // variable name
        // ;
        let mut peek = tokens_iter.peek().cloned();
        loop {
            match peek {
                Some(p) => {
                    if (p.token_str == "static") | (p.token_str == "field") {
                        self.save_to_output("<classVarDec>");
                        self.output_padding += 2;
                        // static or field
                        self.process_type(tokens_iter, TokenType::Keyword);
                        // variable type
                        self.process_next(tokens_iter);
                        self.process_class_variable_names(tokens_iter);

                        self.output_padding -= 2;
                        self.save_to_output("</classVarDec>");
                        peek = tokens_iter.peek().cloned();
                    } else {
                        return;
                    }
                }
                None => {
                    return;
                }
            }
        }
    }

    fn process_class_variable_names<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.process_type(tokens_iter, TokenType::Identifier);

        let next_symbol = tokens_iter.next().unwrap();
        if next_symbol.token_str == "," {
            self.save_to_output(&next_symbol.to_string());
            self.process_class_variable_names(tokens_iter);
        } else if next_symbol.token_str == ";" {
            self.save_to_output(&next_symbol.to_string());
        } else {
            panic!("Found a symbol we shouldn't be seeing {}", next_symbol);
        }
    }

    pub fn process_specific<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut I,
        expected_token_str: String,
        expected_token_type: TokenType,
    ) {
        let next_token = tokens_iter.next().unwrap();
        let expected_token = Token::new(expected_token_str, expected_token_type);

        if *next_token != expected_token {
            panic!(
                "{}\nThe token should be a '{}'. Found '{}'",
                self.file_path.display(),
                expected_token,
                next_token
            )
        }
        self.save_to_output(&next_token.to_string());
    }

    pub fn process_type<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut I,
        expected_token_type: TokenType,
    ) {
        let next_token = tokens_iter.next().unwrap();

        if next_token.token_type != expected_token_type {
            panic!(
                "{}\nThe token should be a '{}'. Found '{}'",
                self.file_path.display(),
                expected_token_type,
                next_token
            )
        }
        self.save_to_output(&next_token.to_string());
    }

    pub fn process_next<'a, I: Iterator<Item = &'a Token>>(&mut self, tokens_iter: &mut I) {
        let next_token = tokens_iter.next().unwrap();
        self.save_to_output(&next_token.to_string());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn save_to_output() {
        let grammar_string = "<class>";
        let mut tabs = String::new();
        let tabs_count = 4;
        for _ in 0..tabs_count {
            tabs += " ";
        }
        assert_eq!(format!("{}{}\n", tabs, grammar_string), "    <class>\n");
    }
}
