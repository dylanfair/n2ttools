use crate::vm::parser::Parser;

impl Parser {
    pub fn handle_goto(&mut self, tokens: Vec<&str>) {
        let goto_label = tokens[1];

        self.output += &format!("@{}${}\n", self.function_name, goto_label);
        self.output += "0;JMP\n";
    }
}
