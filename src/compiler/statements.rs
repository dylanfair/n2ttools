use std::iter::Peekable;

use crate::compiler::parser::Compiler;
use crate::compiler::tokens::Token;

use super::tokens::{self, TokenType};

impl Compiler {
    pub fn process_statements<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        println!("Processing statements");
        self.save_to_output("<statements>");
        self.output_padding += 2;

        let mut next_token = tokens_iter
            .peek()
            .expect("Should not be at end of tokens yet");

        loop {
            println!("In the loop now");
            println!("Token is {}", next_token);
            match next_token.token_str.as_str() {
                "let" => self.process_let_statement(tokens_iter),
                "if" => self.process_if_statement(tokens_iter),
                "while" => self.process_while_statement(tokens_iter),
                "do" => self.process_do_statement(tokens_iter),
                "return" => self.process_return_statement(tokens_iter),
                _ => break,
            }
            next_token = tokens_iter
                .peek()
                .expect("Should not be at the end of tokens yet - loop")
        }

        self.output_padding -= 2;
        self.save_to_output("</statements>");
    }

    pub fn process_let_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<letStatement>");
        self.output_padding += 2;

        // should be a let keyword
        self.process_specific(tokens_iter, String::from("let"), TokenType::Keyword);

        // should be var name
        self.process_var_name(tokens_iter);

        // need to check for an expression here if we see a '['
        let next_token = tokens_iter.next().unwrap();
        if next_token.token_str == "[" {
            self.save_to_output("<symbol> [ </symbol>");

            // parse expression
            self.process_expression(tokens_iter);

            self.process_specific(tokens_iter, String::from("]"), TokenType::Symbol);
        } else if next_token.token_str == "=" {
            // should be =
            self.save_to_output("<symbol> = </symbol>");
        } else {
            panic!("There should only be [ or = here {}", next_token);
        }

        // parse expression
        self.process_expression(tokens_iter);

        // parse ;
        self.process_specific(tokens_iter, String::from(";"), TokenType::Symbol);

        self.output_padding -= 2;
        self.save_to_output("</letStatement>");
    }

    pub fn process_var_name<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        let next = tokens_iter.next().unwrap();

        if next.token_type != TokenType::Identifier {
            panic!("Found '{}' that should have been an Identifier", next);
        }

        self.save_to_output(&format!("<varName> {} </varName>", next.token_str));
    }

    pub fn process_if_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<ifStatement>");
        self.output_padding += 2;

        self.output_padding -= 2;
        self.save_to_output("</ifStatement>");
    }

    pub fn process_while_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<whileStatement>");
        self.output_padding += 2;

        self.output_padding -= 2;
        self.save_to_output("</whileStatement>");
    }

    pub fn process_do_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<doStatement>");
        self.output_padding += 2;

        self.process_subroutinecall(tokens_iter);

        self.output_padding -= 2;
        self.save_to_output("</doStatement>");
    }

    pub fn process_return_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<returnStatement>");
        self.output_padding += 2;

        self.output_padding -= 2;
        self.save_to_output("</returnStatement>");
    }

    pub fn process_expression<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<expression>");
        self.output_padding += 2;

        self.process_term(tokens_iter);

        // check for op and more terms
        let ops = ["+", "-", "*", "/", "&", "|", "<", ">", "="];
        let mut op_peek = tokens_iter.peek().unwrap();
        loop {
            if !ops.contains(&op_peek.token_str.as_str()) {
                break;
            }
            self.process_type(tokens_iter, TokenType::Symbol);
            self.process_term(tokens_iter);
            op_peek = tokens_iter.peek().unwrap();
        }

        self.output_padding -= 2;
        self.save_to_output("</expression>");
    }

    pub fn process_term<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // Need to deal with
        // x IntegerConstant
        // x StringConstant
        // x keywordConstant
        // varName
        // varName[expression]
        // x (expression)
        // x unaryOp term
        // subroutineCall

        // get term
        let next = tokens_iter.next().unwrap();
        self.save_to_output("<term>");
        self.output_padding += 2;

        // IntegerConstant, StringConstant
        // or Keyword constant (like this, true, null, or false)
        if (next.token_type == TokenType::StringConstant)
            | (next.token_type == TokenType::IntegerConstant)
            | (next.token_type == TokenType::Keyword)
        {
            self.save_to_output(&next.to_string());
        }

        // for handling our expression if it's wrapped in parenthesis
        // (expression)
        if next.token_str == "(" {
            self.save_to_output("<symbol> ( </symbol>");
            self.process_expression(tokens_iter);
            self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
        }

        // unaryOp term
        if (next.token_str == "-") | (next.token_str == "~") {
            self.save_to_output(&next.to_string());
            self.process_term(tokens_iter);
        }

        // if we get an identifier, we have to see if it becomes either
        // varName
        // varName[expression]
        // or subroutineCall
        if next.token_type == TokenType::Identifier {
            self.save_to_output(&next.to_string());

            let peek = tokens_iter.peek().unwrap();
            match peek.token_str.as_str() {
                "[" => {
                    self.process_specific(tokens_iter, String::from("["), TokenType::Symbol);
                    self.process_expression(tokens_iter);
                    self.process_specific(tokens_iter, String::from("]"), TokenType::Symbol);
                }
                "(" => {
                    self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                    self.process_expression_list(tokens_iter);
                    self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
                }
                "." => {
                    self.process_specific(tokens_iter, String::from("."), TokenType::Symbol);
                    self.process_type(tokens_iter, TokenType::Identifier);
                    self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                    self.process_expression_list(tokens_iter);
                    self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
                }
                _ => {}
            }
        }

        self.output_padding -= 2;
        self.save_to_output("</term>");
    }

    pub fn process_expression_list<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<expressionList>");
        self.output_padding += 2;

        let mut peek = tokens_iter.peek().unwrap();
        loop {
            if peek.token_str == ")" {
                // meaning expression list has ended
                break;
            }
            self.process_expression(tokens_iter);
            peek = tokens_iter.peek().unwrap();
        }

        self.output_padding -= 2;
        self.save_to_output("</expressionList>");
    }

    pub fn process_subroutinecall<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // do
        self.process_type(tokens_iter, TokenType::Keyword);
        // class, var or subroutine Name
        self.process_type(tokens_iter, TokenType::Identifier);

        let peek = tokens_iter.peek().unwrap();
        match peek.token_str.as_str() {
            "(" => {
                self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                self.process_expression_list(tokens_iter);
                self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
            }
            "." => {
                self.process_specific(tokens_iter, String::from("."), TokenType::Symbol);
                self.process_type(tokens_iter, TokenType::Identifier);
                self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                self.process_expression_list(tokens_iter);
                self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
            }
            _ => {}
        }
        self.process_specific(tokens_iter, String::from(";"), TokenType::Symbol);
    }
}
