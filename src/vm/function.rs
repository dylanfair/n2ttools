use crate::vm::parser::Parser;

impl Parser {
    fn push_address_to_stack(&mut self, address: &str) {
        self.output += &format!("@{}\n", address);
        self.output += "D=M\n";

        self.push_d();
    }

    pub fn handle_function(&mut self, tokens: Vec<&str>) {
        let fn_name = tokens[1];
        let local_variables = tokens[2].parse::<u32>().unwrap();
        self.function_name = fn_name.to_string();

        // (file_name.function_Name)
        self.output += &format!("({})\n", fn_name);

        // initialize local with 0s
        for i in 0..local_variables {
            // get addy of next LCL
            self.output += "@LCL\n";
            self.output += "D=M\n";
            self.output += &format!("@{}\n", i);
            self.output += "D=D+A\n";
            self.output += "@R15\n";
            self.output += "M=D\n";

            // store a 0
            self.output += &format!("@{}\n", 0);
            self.output += "D=A\n";
            self.output += "@R15\n";
            self.output += "A=M\n";
            self.output += "M=D\n";
        }
    }

    pub fn handle_call(&mut self, tokens: Vec<&str>) {
        let fn_name = tokens[1];
        let arg_count = tokens[2].parse::<u32>().unwrap();

        // generate a label and push to stack
        self.output += &format!("@{}$ret.{}\n", fn_name, self.caller_return_number);
        self.output += "D=A\n";
        self.push_d();

        self.push_address_to_stack("LCL");
        self.push_address_to_stack("ARG");
        self.push_address_to_stack("THIS");
        self.push_address_to_stack("THAT");

        // ARG = SP - 5 - arg_count
        self.output += "@SP\n";
        self.output += "D=M\n";
        self.output += &format!("@{}\n", 5 + arg_count);
        self.output += "D=D-A\n";
        self.output += "@ARG\n";
        self.output += "M=D\n";

        // LCL = SP
        self.output += "@SP\n";
        self.output += "D=M\n";
        self.output += "@LCL\n";
        self.output += "M=D\n";

        // @ go to function
        self.output += &format!("@{}\n", fn_name);
        self.output += "0;JMP\n";

        // put a return address
        self.output += &format!("({}$ret.{})\n", fn_name, self.caller_return_number);

        // up call return number
        self.caller_return_number += 1;
    }

    /// Push local to stack
    /// then go to return address
    pub fn handle_return(&mut self, tokens: Vec<&str>) {
        // frame = LCL
        // retAddr = *(frame-5)
        // *ARG = pop()
        // SP = ARG+1
        self.output += "@ARG\n";
        self.output += "D=M\n";
        self.output += "@1\n";
        self.output += "D=D+A\n";
        self.output += "@SP";
        self.output += "M=D";
        // THAT = *(frame-1)
        // THIS = *(frame-2)
        // ARG = *(frame-3)
        // LCL = *(frame-4)
        // goto retAddr

        // file_name.function_name$ret.caller_return_number
        self.output += &format!(
            "@{}$ret.{}\n",
            self.function_name, self.caller_return_number
        );
        self.output += "0;JMP\n";
    }
}
