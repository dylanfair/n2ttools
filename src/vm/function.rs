use crate::vm::parser::Parser;

impl Parser {
    pub fn handle_function(&mut self, tokens: Vec<&str>) {
        let fn_name = tokens[1];
        let arg_count = tokens[2].parse::<u32>().unwrap();
        self.function_name = fn_name.to_string();

        // (file_name.function_Name)
        self.output += &format!("({}.{})\n", self.file_name, self.function_name);
    }

    pub fn handle_return(&mut self, tokens: Vec<&str>) {
        // file_name.function_name$ret.caller_return_number
        self.output += &format!(
            "{}.{}$ret.{}\n",
            self.file_name, self.function_name, self.caller_return_number
        );
        self.output += "0;JMP\n";
    }
}
