use std::iter::Peekable;

use crate::compiler::parser::Compiler;

use super::{
    symbol_table::{SymbolCategory, SymbolTable},
    tokens::{Token, TokenType},
};

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
        self.class_type = self.process_type(&mut tokens_iter, TokenType::Identifier);
        // {
        self.process_specific(&mut tokens_iter, String::from("{"), TokenType::Symbol);
        // class variable declarations
        self.process_class_variable_declarations(&mut tokens_iter);
        if self.debug {
            println!("Class Symbol Table:");
            println!("{:?}", self.class_symbol_table);
        }

        // subroutine declarations
        self.process_subroutine_declarations(&mut tokens_iter);

        // parameter list
        // subroutine body
        // variable declarations

        self.process_specific(&mut tokens_iter, String::from("}"), TokenType::Symbol);
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
                        // need to reset symbol table for subroutine
                        self.subroutine_symbol_table = SymbolTable::new();
                        if p.token_str == "method" {
                            self.subroutine_symbol_table.insert_symbol(
                                String::from("this"),
                                self.class_type.clone(),
                                String::from("arg"),
                                0,
                            );
                            self.subroutine_symbol_table.increment_index("arg");
                        }

                        self.save_to_output("<subroutineDec>");
                        self.output_padding += 2;

                        // process constructor, function or method
                        self.process_type(tokens_iter, TokenType::Keyword);
                        // the type associated with the function
                        self.process_next(tokens_iter);

                        // name of the function
                        let name = self.process_type(tokens_iter, TokenType::Identifier);
                        self.code += &format!("function {}.{}", self.class_type, name);

                        // parameters
                        self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                        self.process_parameter_list(tokens_iter);
                        self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                        // subroutineBody
                        self.process_subroutine_body(tokens_iter);

                        self.output_padding -= 2;
                        self.save_to_output("</subroutineDec>");
                        peek = tokens_iter.peek().cloned();

                        if self.debug {
                            println!("Subroutine symbol table:");
                            println!("{:?}", self.subroutine_symbol_table);
                        }
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
        let mut vars = 0;
        for symbol in self.subroutine_symbol_table.table.values() {
            if let SymbolCategory::Var = symbol.kind {
                vars += 1;
            }
        }
        self.code += &format!(" {}\n", vars);

        // Then iterate through statements
        self.process_statements(tokens_iter);
        self.process_specific(tokens_iter, String::from("}"), TokenType::Symbol);

        self.output_padding -= 2;
        self.save_to_output("</subroutineBody>");
    }

    fn process_subroutine_variable_declarations<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // if we see 'var' then we process next 4 tokens, repeating
        // var
        // type
        // variable name
        // ;
        let mut peek = tokens_iter.peek().cloned();
        loop {
            match peek {
                Some(p) => {
                    if p.token_str == "var" {
                        // we want to get for this particular kind (static, field, var arg) index
                        let mut current_index =
                            self.subroutine_symbol_table.get_index(&p.token_str);

                        self.save_to_output("<varDec>");
                        self.output_padding += 2;
                        // var
                        let token_kind = self.process_type(tokens_iter, TokenType::Keyword);
                        // variable type
                        let token_type = self.process_next(tokens_iter);
                        // names
                        let token_names = self.process_variable_names(tokens_iter);

                        for name in token_names {
                            // push into symbol table
                            self.subroutine_symbol_table.insert_symbol(
                                name,
                                token_type.clone(),
                                token_kind.clone(),
                                current_index,
                            );

                            current_index += 1;
                            self.subroutine_symbol_table.increment_index(&p.token_str);
                        }

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
        let mut current_index = self.subroutine_symbol_table.get_index("arg");
        loop {
            // parameter type
            let token_type = self.process_next(tokens_iter);
            // paramter name
            let token_name = self.process_type(tokens_iter, TokenType::Identifier);

            // add to symbol table
            self.subroutine_symbol_table.insert_symbol(
                token_name,
                token_type,
                String::from("arg"),
                current_index,
            );

            // incremnet index
            current_index += 1;
            self.subroutine_symbol_table.increment_index("arg");

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
                        // we want to get for this particular kind (static, field, var arg) index
                        let mut current_index = self.class_symbol_table.get_index(&p.token_str);

                        self.save_to_output("<classVarDec>");
                        self.output_padding += 2;
                        // static or field
                        let token_kind = self.process_type(tokens_iter, TokenType::Keyword);
                        // variable type
                        let token_type = self.process_next(tokens_iter);
                        // variable name
                        let token_names = self.process_variable_names(tokens_iter);

                        for name in token_names {
                            // push into symbol table
                            self.class_symbol_table.insert_symbol(
                                name,
                                token_type.clone(),
                                token_kind.clone(),
                                current_index,
                            );

                            current_index += 1;
                            self.class_symbol_table.increment_index(&p.token_str);
                        }

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

    fn process_variable_names<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) -> Vec<String> {
        let mut names = vec![];

        let name = self.process_type(tokens_iter, TokenType::Identifier);
        names.push(name);

        let mut next_symbol = tokens_iter.next().unwrap();
        loop {
            if next_symbol.token_str == "," {
                // deal with comma
                self.save_to_output(&next_symbol.to_string());
                // deal with name
                let name = self.process_type(tokens_iter, TokenType::Identifier);
                names.push(name);
                // check for next token
                next_symbol = tokens_iter.next().unwrap();
            } else if next_symbol.token_str == ";" {
                self.save_to_output(&next_symbol.to_string());
                break;
            } else {
                panic!("Found a symbol we shouldn't be seeing {}", next_symbol);
            }
        }

        names
    }

    pub fn process_specific<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut I,
        expected_token_str: String,
        expected_token_type: TokenType,
    ) -> String {
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
        next_token.token_str.clone()
    }

    pub fn process_type<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut I,
        expected_token_type: TokenType,
    ) -> String {
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
        next_token.token_str.clone()
    }

    pub fn process_next<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut I,
    ) -> String {
        let next_token = tokens_iter.next().unwrap();
        self.save_to_output(&next_token.to_string());
        next_token.token_str.clone()
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
