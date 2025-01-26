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
        if s == "push" {
            return Ok(Self::Push);
        }
        if s == "pop" {
            return Ok(Self::Pop);
        }
        return Ok(Self::Arithmetic);
    }
}
