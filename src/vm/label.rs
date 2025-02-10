use crate::vm::parser::Parser;

impl Parser {
    pub fn handle_label(&mut self, tokens: Vec<&str>) {
        let label_name = tokens[1];
        self.output += &format!(
            "({}.{}${})\n",
            self.file_name, self.function_name, label_name
        );
    }
}
