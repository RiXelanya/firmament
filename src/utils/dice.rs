use nom::{sequence::tuple, bytes::complete::tag_no_case, character::complete::{multispace0, digit1}, IResult};
use rand::distributions::{
    Distribution,
    Uniform
};

fn parse_dice(input : &str) -> IResult<&str, (&str,&str)> {
    let (remainder , (_,dice_number,_,dice_rank)) = tuple((
        multispace0,
        digit1,
        tag_no_case("d"),
        digit1
    ))(input)?;
    Ok((remainder, (dice_number,dice_rank)))

}

pub fn roll(input : &str) -> Vec<u8> {
    let (_ , (dice_number,dice_rank)) = parse_dice(input).unwrap() ;
    let dice_rank: u8 = dice_rank.parse::<u8>().unwrap() + 1 ;
    let dice_number: u8 = dice_number.parse::<u8>().unwrap() ;
    let mut output: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(1..dice_rank);
    for _ in 1..=dice_number {
        output.push(die.sample(&mut rng));
    }
    output
}