use nom::{
    bytes::complete::tag,
    combinator::{map, map_res, value},
    number::{
        complete::{double, float},
        streaming::f64,
    },
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Subtraction,
    Addition,
    Multiplication,
    Division,
    Exponentiation,
    OpeningParenthesis,
    ClosingParenthesis,
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    kind: TokenKind,
}

fn parse_sub(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::Subtraction,
        },
        tag("-"),
    )(input)
}

fn parse_add(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::Addition,
        },
        tag("+"),
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::Multiplication,
        },
        tag("*"),
    )(input)
}

fn parse_div(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::Division,
        },
        tag("/"),
    )(input)
}

fn parse_exp(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::Exponentiation,
        },
        tag("^"),
    )(input)
}

fn parse_open_paren(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::OpeningParenthesis,
        },
        tag("("),
    )(input)
}

fn parse_close_paren(input: &str) -> IResult<&str, Token> {
    value(
        Token {
            kind: TokenKind::ClosingParenthesis,
        },
        tag(")"),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    map(double, |res| Token {
        kind: TokenKind::Number(res),
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("-10")]
    fn test_sub(#[case] input: &str) {
        let res = parse_sub(input);

        assert_eq!(
            res,
            Ok((
                "10",
                Token {
                    kind: TokenKind::Subtraction,
                }
            ))
        );
    }

    #[rstest]
    #[case("+10")]
    fn test_add(#[case] input: &str) {
        let res = parse_add(input);

        assert_eq!(
            res,
            Ok((
                "10",
                Token {
                    kind: TokenKind::Addition,
                }
            ))
        );
    }

    #[rstest]
    #[case("*10")]
    fn test_mul(#[case] input: &str) {
        let res = parse_mul(input);

        assert_eq!(
            res,
            Ok((
                "10",
                Token {
                    kind: TokenKind::Multiplication,
                }
            ))
        );
    }

    #[rstest]
    #[case("/10")]
    fn test_div(#[case] input: &str) {
        let res = parse_div(input);

        assert_eq!(
            res,
            Ok((
                "10",
                Token {
                    kind: TokenKind::Division,
                }
            ))
        );
    }

    #[rstest]
    #[case("^10")]
    fn test_exp(#[case] input: &str) {
        let res = parse_exp(input);

        assert_eq!(
            res,
            Ok((
                "10",
                Token {
                    kind: TokenKind::Exponentiation,
                }
            ))
        );
    }

    #[rstest]
    #[case("(")]
    fn test_open_paren(#[case] input: &str) {
        let res = parse_open_paren(input);

        assert_eq!(
            res,
            Ok((
                "",
                Token {
                    kind: TokenKind::OpeningParenthesis,
                }
            ))
        );
    }

    #[rstest]
    #[case(")")]
    fn test_close_paren(#[case] input: &str) {
        let res = parse_close_paren(input);

        assert_eq!(
            res,
            Ok((
                "",
                Token {
                    kind: TokenKind::ClosingParenthesis,
                }
            ))
        );
    }

    #[rstest]
    #[case("10.0", 10., "")]
    #[case("10.0+", 10., "+")]
    #[case("10.0+1", 10., "+1")]
    fn test_number(#[case] input: &str, #[case] output: f64, #[case] remainder: &str) {
        let res = parse_number(input);

        assert_eq!(
            res,
            Ok((
                remainder,
                Token {
                    kind: TokenKind::Number(output),
                }
            ))
        );
    }
}
