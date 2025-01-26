pub fn handle_arithmetic(tokens: Vec<&str>) -> String {
    let arithmetic = tokens[0];
    let mut output = match arithmetic {
        "add" => add(),
        "sub" => sub(),
        "neg" => neg(),
        "eq" => eq(),
        "gt" => gt(),
        "lt" => lt(),
        "and" => and(),
        "or" => or(),
        "not" => not(),
        _ => panic!("ran into a different arithmetic"),
    };

    // then move up one of the stack
    output += "@SP\n";
    output += "M=M+1\n";
    output
}

fn get_y() -> String {
    // go down stack and set first value to D (y)
    let mut output = String::from("@SP\n");
    output += "M=M-1\n";
    output += "@SP\n";
    output += "A=M\n";
    output += "D=M\n";
    output
}

fn get_values() -> String {
    let mut output = get_y();

    // now go down again and prep for x
    output += "@SP\n";
    output += "M=M-1\n";
    output += "A=M\n";
    output
}

fn true_or_false() -> String {
    // True
    let mut output = String::from("(TRUE)\n");

    output += "@R15\n";
    output += "M=D\n";

    output += "@-1\n";
    output += "D=A\n";
    output += "@SP\n";
    output += "A=M\n";
    output += "M=D\n";

    output += "@R15\n";
    output += "0;JMP\n";

    // False
    output += "(FALSE)\n";

    output += "@R15\n";
    output += "M=D\n";

    output += "@0\n";
    output += "D=A\n";
    output += "@SP\n";
    output += "A=M\n";
    output += "M=D\n";

    output += "@R15\n";
    output += "0;JMP\n";

    output
}

fn add() -> String {
    let mut output = get_values();

    // now add values
    // x + y
    output += "M=D+M\n";
    output
}

fn sub() -> String {
    let mut output = get_values();

    // now sub values
    // x - y
    output += "M=M-D\n";
    output
}

fn neg() -> String {
    let mut output = get_y();

    // negative y
    output += "M=-D\n";
    output
}

fn eq() -> String {
    let mut output = get_values();

    // set a return address
    output += "@returnaddress\n";
    output += "D=A\n";

    output += "@TRUE\n";
    output += "D-M;JEQ\n";
    output += "@FALSE\n";
    output += "0;JMP\n";

    output += &true_or_false();

    output
}

fn gt() -> String {
    let mut output = get_values();

    output += "@returnaddress\n";
    output += "D=A\n";

    output += "@TRUE\n";
    output += "D-M;JGT\n";
    output += "@FALSE\n";
    output += "0;JMP\n";

    output += &true_or_false();

    output
}

fn lt() -> String {
    let mut output = get_values();

    output += "@returnaddress\n";
    output += "D=A\n";

    output += "@TRUE\n";
    output += "D-M;JLT\n";
    output += "@FALSE\n";
    output += "0;JMP\n";

    output += &true_or_false();

    output
}

fn and() -> String {
    let mut output = get_values();

    output += "M=D&M\n";
    output
}

fn or() -> String {
    let mut output = get_values();

    output += "M=D|M\n";
    output
}

fn not() -> String {
    let mut output = get_y();

    output += "M=!D\n";
    output
}
