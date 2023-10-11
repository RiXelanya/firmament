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

pub fn interpret(input : &str) -> Command {
    let(command,prefix) = parse(input).unwrap();
    match prefix {
        "!" => imperial::interpret(command),
        _ => panic!()
    }
}