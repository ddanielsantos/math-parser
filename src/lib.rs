use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, value},
    error::ParseError,
    multi::many0,
    number::complete::double,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Subtraction,
    Addition,
    Multiplication,
    Division,
    Exponentiation,
    OpeningParenthesis,
    ClosingParenthesis,
    Number(f64),
}

fn ws_eater<'i, O, E: ParseError<&'i str>, F>(inner: F) -> impl Parser<&'i str, O, E>
where
    F: Parser<&'i str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_add,
        parse_sub,
        parse_mul,
        parse_div,
        parse_exp,
        parse_open_paren,
        parse_close_paren,
        parse_number,
    ))(input)
}

fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(parse_token)(input)
}

fn parse_sub(input: &str) -> IResult<&str, Token> {
    value(Token::Subtraction, ws_eater(tag("-")))(input)
}

fn parse_add(input: &str) -> IResult<&str, Token> {
    value(Token::Addition, ws_eater(tag("+")))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Token> {
    value(Token::Multiplication, ws_eater(tag("*")))(input)
}

fn parse_div(input: &str) -> IResult<&str, Token> {
    value(Token::Division, ws_eater(tag("/")))(input)
}

fn parse_exp(input: &str) -> IResult<&str, Token> {
    value(Token::Exponentiation, ws_eater(tag("^")))(input)
}

fn parse_open_paren(input: &str) -> IResult<&str, Token> {
    value(Token::OpeningParenthesis, ws_eater(tag("(")))(input)
}

fn parse_close_paren(input: &str) -> IResult<&str, Token> {
    value(Token::ClosingParenthesis, ws_eater(tag(")")))(input)
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    map(ws_eater(double), Token::Number).parse(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("-10")]
    fn test_sub(#[case] input: &str) {
        let res = parse_sub(input);

        assert_eq!(res, Ok(("10", Token::Subtraction)));
    }

    #[rstest]
    #[case("+10")]
    fn test_add(#[case] input: &str) {
        let res = parse_add(input);

        assert_eq!(res, Ok(("10", Token::Addition)));
    }

    #[rstest]
    #[case("*10")]
    fn test_mul(#[case] input: &str) {
        let res = parse_mul(input);

        assert_eq!(res, Ok(("10", Token::Multiplication)));
    }

    #[rstest]
    #[case("/10")]
    fn test_div(#[case] input: &str) {
        let res = parse_div(input);

        assert_eq!(res, Ok(("10", Token::Division)));
    }

    #[rstest]
    #[case("^10")]
    fn test_exp(#[case] input: &str) {
        let res = parse_exp(input);

        assert_eq!(res, Ok(("10", Token::Exponentiation)));
    }

    #[rstest]
    #[case("(")]
    fn test_open_paren(#[case] input: &str) {
        let res = parse_open_paren(input);

        assert_eq!(res, Ok(("", Token::OpeningParenthesis)));
    }

    #[rstest]
    #[case(")")]
    fn test_close_paren(#[case] input: &str) {
        let res = parse_close_paren(input);

        assert_eq!(res, Ok(("", Token::ClosingParenthesis)));
    }

    #[rstest]
    #[case("10.0", 10., "")]
    #[case("10.0+", 10., "+")]
    #[case("10.0+1", 10., "+1")]
    fn test_number(#[case] input: &str, #[case] output: f64, #[case] remainder: &str) {
        let res = parse_number(input);

        assert_eq!(res, Ok((remainder, Token::Number(output))));
    }

    #[rstest]
    #[case("1+1")]
    #[case("1 + 1")]
    fn test_tokenizer(#[case] input: &str) {
        let res = tokenize(input);

        assert_eq!(
            res,
            Ok((
                "",
                vec![Token::Number(1.), Token::Addition, Token::Number(1.)]
            ))
        );
    }
}
