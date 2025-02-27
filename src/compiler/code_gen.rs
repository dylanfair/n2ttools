use crate::compiler::parser::Compiler;

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
            _ => panic!("Got an invalid operator: '{}'", operator_str),
        };
        self.write_code(vm_operator);
    }

    pub fn compile_constant(&mut self, constant: &str) {
        self.write_code(&format!("push constant {}", constant));
    }

    pub fn compile_return(&mut self) {
        self.write_code("return");
    }
}
