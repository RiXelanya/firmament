use nom::{
    branch::alt,
    bytes::complete::tag,
    IResult,
};

use crate::Command;

mod imperial ;

fn parse(input : &str) -> IResult<&str,&str> {
    alt((
        tag("!"),
    ))(input)
}

pub fn interpret(input : &str) -> Option<Command> {
    let output = parse(input);
    match output {
        Err(_) => None,
        Ok((command,prefix)) => match prefix {
                "!" => Some(imperial::interpret(command)),
                _ => None
            }
    }
}