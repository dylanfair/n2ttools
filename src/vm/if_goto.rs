use crate::vm::parser::Parser;

impl Parser {
    /// Pop topmost value off the stack
    /// if it is true
    /// jump to the label
    pub fn handle_if_goto(&mut self, tokens: Vec<&str>) {
        let label = tokens[1];
        // get top-most value off stack
        self.pop_stack();
        // value now stored in D

        // check if value is true
        // i.e. D == 0
        self.output += &format!("@{}${}\n", self.function_name, label);
        self.output += "D;JNE\n";
    }
}
