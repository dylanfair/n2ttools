use crate::compiler::parser::Compiler;

use super::symbol_table::Symbol;

impl Compiler {
    pub fn write_code(&mut self, code_str: &str) {
        self.code += &format!("{}\n", code_str);
    }

    pub fn compile_math_operator(&mut self, operator_str: &str) {
        let vm_operator = match operator_str {
            "+" => "add",
            "-" => "sub",
            "*" => "call Math.multiply 2",
            "/" => "call Math.divide 2",
            "&gt;" => "gt",
            "&lt;" => "lt",
            "=" => "eq",
            "&amp;" => "and",
            "|" => "or",
            _ => panic!("Got an invalid operator: '{}'", operator_str),
        };
        self.write_code(vm_operator);
    }

    pub fn compile_constant(&mut self, constant: &str) {
        self.write_code(&format!("push constant {}", constant));
    }

    pub fn compile_string(&mut self, string: &str) {
        let string_len = string.len();
        self.write_code(&format!("push constant {}", string_len));
        self.write_code("call String.new 1");

        for c in string.chars() {
            let char_code = self
                .character_set
                .get(&c.to_string())
                .expect("Shouldn't see other chars");

            self.write_code(&format!("push constant {}", char_code));
            self.write_code("call String.appendChar 2");
        }
    }

    pub fn compile_return(&mut self) {
        self.write_code("return");
    }

    pub fn compile_keyword(&mut self, keyword: &str) {
        match keyword {
            "true" => {
                self.write_code("push constant 0");
                self.write_code("not");
            }
            "false" => self.write_code("push constant 0"),
            "null" => todo!(),
            "this" => self.write_code("push pointer 0"), // when returning this
            _ => panic!(
                "Came across an unfamiliar keyword we shouldn't see: '{}'",
                keyword
            ),
        }
    }

    pub fn compile_unary_op(&mut self, op: &str) {
        match op {
            "-" => self.write_code("neg"),
            "~" => self.write_code("not"),
            _ => panic!("Was given an op that wasn't unary: '{}'", op),
        }
    }

    pub fn pop_symbol(&mut self, symbol: &Symbol) {
        let kind = symbol.kind.to_string();
        let index = symbol.index;
        self.write_code(&format!("pop {} {}", kind, index))
    }

    pub fn push_symbol(&mut self, symbol: &Symbol) {
        let kind = symbol.kind.to_string();
        let index = symbol.index;
        self.write_code(&format!("push {} {}", kind, index))
    }
}
