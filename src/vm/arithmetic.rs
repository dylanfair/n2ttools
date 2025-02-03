use crate::vm::parser::Parser;

impl Parser {
    pub fn handle_arithmetic(&mut self, tokens: Vec<&str>) {
        let arithmetic = tokens[0];
        match arithmetic {
            "add" => self.add(),
            "sub" => self.sub(),
            "neg" => self.neg(),
            "eq" => self.eq(),
            "gt" => self.gt(),
            "lt" => self.lt(),
            "and" => self.and(),
            "or" => self.or(),
            "not" => self.not(),
            _ => panic!("ran into a different arithmetic"),
        };

        // then move up one of the stack
        self.output += "@SP\n";
        self.output += "M=M+1\n";
    }

    fn get_y(&mut self) {
        // go down stack and set first value to D (y)
        self.output += "@SP\n";
        self.output += "M=M-1\n";
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "D=M\n";
    }

    fn get_values(&mut self) {
        self.get_y();

        // now go down again and prep for x
        self.output += "@SP\n";
        self.output += "M=M-1\n";
        self.output += "A=M\n";
    }

    fn true_or_false(&mut self) {
        // True
        self.output += &format!("(TRUE_{})\n", self.return_caller_number);
        self.output += "@0\n";
        self.output += "D=A\n";
        self.output += "D=D-1\n";
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "M=D\n";

        self.output += &format!("@RETURN_ADDRESS_{}\n", self.return_caller_number);
        self.output += "0;JMP\n";

        // False
        self.output += &format!("(FALSE_{})\n", self.return_caller_number);
        self.output += "@0\n";
        self.output += "D=A\n";
        self.output += "@SP\n";
        self.output += "A=M\n";
        self.output += "M=D\n";

        self.output += &format!("@RETURN_ADDRESS_{}\n", self.return_caller_number);
        self.output += "0;JMP\n";
    }

    fn add(&mut self) {
        self.get_values();

        // now add values
        // x + y
        // A is set to current stack and we replace value
        self.output += "M=D+M\n";
    }

    fn sub(&mut self) {
        self.get_values();

        // now sub values
        // x - y
        self.output += "M=M-D\n";
    }

    fn neg(&mut self) {
        self.get_y();

        // negative y
        self.output += "M=-D\n";
    }

    fn eq(&mut self) {
        // subroutine calling sequence
        // @returnaddress
        // D=A
        // @subroutine
        // 0;JMP
        //
        // suboutine ; D contains return address
        // subroutine entry code
        // @STK
        // AM=M+1 ; bumps the stack pointer, also setting A to new SP value
        // M=D ; write the return address into the stack
        // *** Now do subroutine work (i.e. check for true or false)
        // subroutine exit code
        // @STK
        // AM=M-1
        // A=M
        // 0;JMP
        self.get_values();
        // A is set to current stack
        // D holds y
        // M now holds X

        // do comparison
        self.output += "D=D-M\n";
        self.output += &format!("@TRUE_{}\n", self.return_caller_number);
        self.output += "D;JEQ\n";
        self.output += &format!("@FALSE_{}\n", self.return_caller_number);
        self.output += "0;JMP\n";

        self.true_or_false();
        self.output += &format!("(RETURN_ADDRESS_{})\n", self.return_caller_number);

        // up our return_caller_number
        self.return_caller_number += 1;
    }

    fn gt(&mut self) {
        self.get_values();
        // D holds y
        // M now holds X

        self.output += "D=M-D\n";
        self.output += &format!("@TRUE_{}\n", self.return_caller_number);
        self.output += "D;JGT\n";
        self.output += &format!("@FALSE_{}\n", self.return_caller_number);
        self.output += "0;JMP\n";

        self.true_or_false();
        self.output += &format!("(RETURN_ADDRESS_{})\n", self.return_caller_number);

        // up our return_caller_number
        self.return_caller_number += 1;
    }

    fn lt(&mut self) {
        self.get_values();
        // D holds y
        // M now holds X

        self.output += "D=M-D\n";
        self.output += &format!("@TRUE_{}\n", self.return_caller_number);
        self.output += "D;JLT\n";
        self.output += &format!("@FALSE_{}\n", self.return_caller_number);
        self.output += "0;JMP\n";

        self.true_or_false();
        self.output += &format!("(RETURN_ADDRESS_{})\n", self.return_caller_number);

        // up our return_caller_number
        self.return_caller_number += 1;
    }

    fn and(&mut self) {
        self.get_values();

        self.output += "M=D&M\n";
    }

    fn or(&mut self) {
        self.get_values();

        self.output += "M=D|M\n";
    }

    fn not(&mut self) {
        self.get_y();

        self.output += "M=!D\n";
    }
}
