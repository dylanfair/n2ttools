use std::iter::Peekable;

use crate::compiler::parser::Compiler;
use crate::compiler::tokens::Token;

use super::tokens::TokenType;

impl Compiler {
    pub fn process_statements<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<statements>");
        self.output_padding += 2;

        let mut next_token = tokens_iter
            .peek()
            .expect("Should not be at end of tokens yet");

        loop {
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
        let var_name = self.process_type(tokens_iter, TokenType::Identifier);

        let var_name_symbol = self.get_symbol(&var_name);
        let symbol = match var_name_symbol {
            Some(symbol) => symbol.clone(),
            None => panic!("Could not find {} in our symbol table", var_name),
        };

        // need to check for an expression here if we see a '['
        let next_token = tokens_iter.next().unwrap();
        if next_token.token_str == "[" {
            self.save_to_output("<symbol> [ </symbol>");

            // parse expression
            self.process_expression(tokens_iter);

            self.process_specific(tokens_iter, String::from("]"), TokenType::Symbol);
            self.process_specific(tokens_iter, String::from("="), TokenType::Symbol);
        } else if next_token.token_str == "=" {
            // should be =
            self.save_to_output("<symbol> = </symbol>");
        } else {
            panic!("There should only be [ or = here {}", next_token);
        }

        // parse expression
        self.process_expression(tokens_iter);
        // now pop our var
        self.pop_symbol(&symbol);

        // parse ;
        self.process_specific(tokens_iter, String::from(";"), TokenType::Symbol);

        self.output_padding -= 2;
        self.save_to_output("</letStatement>");
    }

    pub fn process_if_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // keep a static counter here for this if loop
        let current_if_counter = self.branches.if_counter;
        self.branches.if_counter += 1;

        self.save_to_output("<ifStatement>");
        self.output_padding += 2;

        // should be a if keyword
        self.process_specific(tokens_iter, String::from("if"), TokenType::Keyword);
        self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);

        self.process_expression(tokens_iter);
        self.write_code(&format!("if-goto IF_TRUE{}", current_if_counter));
        self.write_code(&format!("goto IF_FALSE{}", current_if_counter));

        self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
        self.process_specific(tokens_iter, String::from("{"), TokenType::Symbol);

        self.write_code(&format!("label IF_TRUE{}", current_if_counter));
        self.process_statements(tokens_iter);
        self.process_specific(tokens_iter, String::from("}"), TokenType::Symbol);

        // now check for else
        let else_peek = tokens_iter.peek().unwrap();
        if else_peek.token_str == "else" {
            self.write_code(&format!("goto IF_END{}", current_if_counter));
            self.process_specific(tokens_iter, String::from("else"), TokenType::Keyword);
            self.process_specific(tokens_iter, String::from("{"), TokenType::Symbol);
            self.write_code(&format!("label IF_FALSE{}", current_if_counter));
            self.process_statements(tokens_iter);
            self.process_specific(tokens_iter, String::from("}"), TokenType::Symbol);
            self.write_code(&format!("label IF_END{}", current_if_counter));
        } else {
            self.write_code(&format!("label IF_FALSE{}", current_if_counter));
        }

        self.output_padding -= 2;
        self.save_to_output("</ifStatement>");
    }

    pub fn process_while_statement<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        let current_while_counter = self.branches.while_counter;
        self.branches.while_counter += 1;

        self.save_to_output("<whileStatement>");
        self.output_padding += 2;

        // should be a while keyword
        self.process_specific(tokens_iter, String::from("while"), TokenType::Keyword);
        self.write_code(&format!("label WHILE_EXP{}", current_while_counter));

        self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
        self.process_expression(tokens_iter);
        self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);
        self.write_code("not");
        self.write_code(&format!("if-goto WHILE_END{}", current_while_counter));

        self.process_specific(tokens_iter, String::from("{"), TokenType::Symbol);
        self.process_statements(tokens_iter);
        self.process_specific(tokens_iter, String::from("}"), TokenType::Symbol);
        self.write_code(&format!("goto WHILE_EXP{}", current_while_counter));

        self.write_code(&format!("label WHILE_END{}", current_while_counter));

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

        self.process_specific(tokens_iter, String::from("return"), TokenType::Keyword);
        let peek = tokens_iter.peek().unwrap();
        if peek.token_str != ";" {
            self.process_expression(tokens_iter);
        } else {
            // if there are no expressions present
            // we push constant 0
            self.write_code("push constant 0");
        }
        self.process_specific(tokens_iter, String::from(";"), TokenType::Symbol);
        self.compile_return();

        self.output_padding -= 2;
        self.save_to_output("</returnStatement>");
    }

    pub fn process_expression<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        self.save_to_output("<expression>");
        self.output_padding += 2;

        // push term in
        self.process_term(tokens_iter);

        // check for op and more terms
        let ops = ["+", "-", "*", "/", "&amp;", "|", "&lt;", "&gt;", "="];
        let mut op_peek = tokens_iter.peek().unwrap();
        loop {
            if !ops.contains(&op_peek.token_str.as_str()) {
                break;
            }
            let operator = self.process_type(tokens_iter, TokenType::Symbol);
            // push second term
            self.process_term(tokens_iter);
            // work operator
            if ops.contains(&operator.as_str()) {
                self.compile_math_operator(&operator);
            }
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
            match next.token_type {
                TokenType::IntegerConstant => self.compile_constant(&next.token_str),
                TokenType::Keyword => self.compile_keyword(&next.token_str),
                TokenType::StringConstant => self.compile_string(&next.token_str),
                _ => panic!("Shouldn't see any other token type here: '{}'", next),
            }
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
            self.compile_unary_op(&next.token_str);
        }

        // if we get an identifier, we have to see if it becomes either
        // varName
        // varName[expression]
        // or subroutineCall
        if next.token_type == TokenType::Identifier {
            self.save_to_output(&next.to_string());

            let mut name = next.token_str.clone();
            let og_name = name.clone();
            let mut expression_count = 0;
            let mut method_call = false;

            let symbol_check = self.get_symbol(&og_name);
            if let Some(symbol) = symbol_check {
                expression_count += 1;
                name = symbol.var_type.clone();
                method_call = true;
            }

            let peek = tokens_iter.peek().unwrap();
            match peek.token_str.as_str() {
                "[" => {
                    // dealing with an array
                    // let x = arr[2]
                    // where arr is local 0
                    // and x is local 1
                    //
                    // array part
                    // push local 0
                    // push constant 2
                    // add
                    // pop pointer 1
                    //
                    // x part
                    // push that 0
                    // pop local 1
                    //
                    // let arr[expression1] = expression2
                    // push local 0 (arr)
                    // compile expresssion1
                    // add
                    // pop pointer 1
                    // compile expression2
                    // pop temp 0
                    // pop pointer 1
                    // push temp 0
                    // pop that 0
                    self.process_specific(tokens_iter, String::from("["), TokenType::Symbol);
                    self.process_expression(tokens_iter);
                    self.process_specific(tokens_iter, String::from("]"), TokenType::Symbol);
                }
                "(" => {
                    self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                    name = format!("{}.{}", self.class_type, name);
                    expression_count += self.process_expression_list(tokens_iter);
                    expression_count += 1; // implies we are using a method of the current object
                    self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                    // if using a method of the current object, need to push in this
                    self.write_code("push pointer 0");
                    self.write_code(&format!("call {} {}", name, expression_count));
                }
                "." => {
                    name +=
                        &self.process_specific(tokens_iter, String::from("."), TokenType::Symbol);
                    name += &self.process_type(tokens_iter, TokenType::Identifier);
                    self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                    expression_count = self.process_expression_list(tokens_iter);
                    self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                    if method_call {
                        // symbol table shouldn't have changed by this point
                        let symbol = self.get_symbol(&og_name).unwrap().clone();
                        self.push_symbol(&symbol);
                    }
                    self.write_code(&format!("call {} {}", name, expression_count));
                }
                _ => {
                    let symbol = self.get_symbol(&og_name);
                    let symbol = match symbol {
                        Some(symbol) => symbol.clone(),
                        None => panic!(
                            "Could not find the following in our symbol tables: '{}'",
                            name
                        ),
                    };
                    self.push_symbol(&symbol);
                }
            }
        }

        self.output_padding -= 2;
        self.save_to_output("</term>");
    }

    pub fn process_expression_list<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) -> i32 {
        self.save_to_output("<expressionList>");
        self.output_padding += 2;

        let mut peek = tokens_iter.peek().unwrap();
        let mut counter = 0;
        loop {
            if peek.token_str == ")" {
                // meaning expression list has ended
                break;
            }
            if counter > 0 {
                self.process_specific(tokens_iter, String::from(","), TokenType::Symbol);
            }
            self.process_expression(tokens_iter);
            peek = tokens_iter.peek().unwrap();
            counter += 1;
        }

        self.output_padding -= 2;
        self.save_to_output("</expressionList>");
        counter
    }

    pub fn process_subroutinecall<'a, I: Iterator<Item = &'a Token>>(
        &mut self,
        tokens_iter: &mut Peekable<I>,
    ) {
        // do
        self.process_type(tokens_iter, TokenType::Keyword);
        // class, var or subroutine Name
        let mut name = self.process_type(tokens_iter, TokenType::Identifier);
        let og_name = name.clone(); // for use later if name gets changed
        let mut method_call = false;
        let mut expression_count = 0;

        // we want to change the name from the variable name
        // to the type if this is the case
        let symbol_check = self.get_symbol(&og_name);
        if let Some(symbol) = symbol_check {
            expression_count += 1;
            name = symbol.var_type.clone();
            method_call = true;
        }

        let peek = tokens_iter.peek().unwrap();
        match peek.token_str.as_str() {
            "(" => {
                self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                name = format!("{}.{}", self.class_type, name);
                expression_count = self.process_expression_list(tokens_iter);
                expression_count += 1; // implies we are using a method of the current object
                self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                // if using a method of the current object, need to push in this
                self.write_code("push pointer 0");
            }
            "." => {
                name += &self.process_specific(tokens_iter, String::from("."), TokenType::Symbol);
                name += &self.process_type(tokens_iter, TokenType::Identifier);
                self.process_specific(tokens_iter, String::from("("), TokenType::Symbol);
                expression_count += self.process_expression_list(tokens_iter);
                self.process_specific(tokens_iter, String::from(")"), TokenType::Symbol);

                if method_call {
                    // symbol table shouldn't have changed by this point
                    let symbol = self.get_symbol(&og_name).unwrap().clone();
                    self.push_symbol(&symbol);
                }
            }
            _ => {}
        }
        self.process_specific(tokens_iter, String::from(";"), TokenType::Symbol);
        self.write_code(&format!("call {} {}", name, expression_count));
        self.write_code("pop temp 0");
    }
}
