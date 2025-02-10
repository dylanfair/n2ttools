use std::str::FromStr;

pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

#[derive(Debug)]
pub struct CommandParseError;

impl FromStr for CommandType {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(Self::Push),
            "pop" => Ok(Self::Pop),
            "label" => Ok(Self::Label),
            "if-goto" => Ok(Self::If),
            "goto" => Ok(Self::Goto),
            _ => Ok(Self::Arithmetic),
        }
    }
}
