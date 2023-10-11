use nom::{
    IResult,
    bytes::complete::tag,
    branch::alt, sequence::tuple, character::complete::multispace0
};

use crate::Command;

mod oracle ;




fn parse(input : &str) -> IResult<&str, &str> {
    let (argument, (command, _)) = tuple((
        alt((
            tag("oracle"),
        )),
        multispace0
    ))(input)? ;
    Ok((argument, command))
    
}

pub fn interpret(input : &str) -> Command {
    let (subcommand, command) = parse(input).unwrap();
    match command {
        "oracle" => oracle::interpret(subcommand),
        _ => panic!()
    }
}