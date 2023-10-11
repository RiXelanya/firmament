#[cfg(test)]
pub mod test {
    use crate::parser ;
    use crate::Command;
    #[test]
    fn parse_oracle_init_works() {
        let input = "!oracle init 3d6" ;
        let command : Command = parser::interpret(input) ;
        assert_eq!(command, Command::OracleInit("3d6"))
    }

    #[test]
    fn parse_oracle_guess_works() {
        let input = "!oracle guess 3" ;
        let command : Command = parser::interpret(input) ;
        assert_eq!(command, Command::OracleGuess("3"))
    }
}