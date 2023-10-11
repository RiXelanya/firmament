use nom::{
    IResult,
    bytes::complete::tag,
    branch::alt, sequence::tuple, character::complete::multispace0
};

use crate::Command;

pub fn interpret(subcommand: &str) -> Command {
    let (argument, subcommand) = parse(subcommand).unwrap();
    match subcommand {
        "init" => Command::OracleInit(argument),
        "guess" => Command::OracleGuess(argument),
        _ => panic!()
    }
}

fn parse(input : &str) -> IResult<&str,&str> {
    let (argument, (subcommand, _)) = tuple((
        alt((
            tag("init"),
            tag("guess")
        )),
        multispace0
    ))(input)? ;
    Ok((argument, subcommand))
}