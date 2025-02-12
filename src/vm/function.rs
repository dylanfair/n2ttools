use crate::vm::parser::Parser;

impl Parser {
    fn push_address_to_stack(&mut self, address: &str) {
        self.output += &format!("@{}\n", address);
        self.output += "D=M\n";

        self.push_d();
    }

    fn get_frame(&mut self, n: u64) {
        self.output += "@R13\n";
        self.output += "D=M\n";
        self.output += &format!("@{}\n", n);
        self.output += "D=D-A\n";
        self.output += "A=D\n";
        self.output += "D=M\n";
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
    pub fn handle_return(&mut self) {
        // frame = LCL - temp variable
        self.output += "@LCL\n";
        self.output += "D=M\n";
        self.output += "@R13\n";
        self.output += "M=D\n";

        // retAddr = *(frame-5)
        self.get_frame(5);
        self.output += "@R14\n";
        self.output += "M=D\n";

        // *ARG = pop()
        // move value from SP to where ARG value is *ARG
        self.pop_stack();
        // move to ARG pointer
        self.output += "@ARG\n";
        self.output += "A=M\n";
        self.output += "M=D\n";

        // SP = ARG+1
        self.output += "@ARG\n";
        self.output += "D=M\n";
        self.output += "@1\n";
        self.output += "D=D+A\n";
        self.output += "@SP\n";
        self.output += "M=D\n";

        // THAT = *(frame-1)
        self.get_frame(1);
        self.output += "@THAT\n";
        self.output += "M=D\n";
        // THIS = *(frame-2)
        self.get_frame(2);
        self.output += "@THIS\n";
        self.output += "M=D\n";
        // ARG = *(frame-3)
        self.get_frame(3);
        self.output += "@ARG\n";
        self.output += "M=D\n";
        // LCL = *(frame-4)
        self.get_frame(4);
        self.output += "@LCL\n";
        self.output += "M=D\n";

        // goto retAddr
        // file_name.function_name$ret.caller_return_number
        self.output += "@R14\n";
        self.output += "A=M\n";
        // self.output += &format!(
        //     "@{}$ret.{}\n",
        //     self.function_name, self.caller_return_number
        // );
        self.output += "0;JMP\n";
    }
}
