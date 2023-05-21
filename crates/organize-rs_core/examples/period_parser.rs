use winnow::{ascii::alpha0, combinator::alt, token::take_while, IResult, Parser};

pub fn parse_garbage(input: &str) -> IResult<&str, &str> {
    take_while(1.., " ,").parse_next(input)
}

pub fn parse_eq_operators(input: &str) -> IResult<&str, &str> {
    take_while(1.., "><=").parse_next(input)
}

pub fn parse_boundary(input: &str) -> IResult<&str, &str> {
    alt((("..="), (".."))).parse_next(input)
}

pub fn parse_digits(input: &str) -> IResult<&str, f64> {
    take_while(1.., (('0'..='9'), ('.')))
        .parse_to()
        .parse_next(input)
}

pub fn parse_right_boundary(input: &str) -> IResult<&str, (&str, f64, &str)> {
    (parse_boundary, parse_digits, parse_units).parse_next(input)
}

pub fn parse_left_boundary(input: &str) -> IResult<&str, (f64, &str, &str)> {
    (parse_digits, parse_units, parse_boundary).parse_next(input)
}

pub fn parse_whole_condition(input: &str) -> IResult<&str, (f64, &str, &str, f64, &str)> {
    (
        parse_digits,
        parse_units,
        parse_boundary,
        parse_digits,
        parse_units,
    )
        .parse_next(input)
}

pub fn parse_whole_period(input: &str) -> IResult<&str, (f64, &str, f64, &str)> {
    (parse_digits, parse_boundary, parse_digits, parse_units).parse_next(input)
}

pub fn parse_units(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn main() {
    let input = "1d..7d";
    let input2 = "7d..";
    let input3 = "..7d";

    let (remainder, result) = (parse_whole_condition).parse_next(input).unwrap();
    // let (_, _result) = (parse_left_boundary).parse_next(input2).unwrap();
    // let (_, _result) = (parse_right_boundary).parse_next(input3).unwrap();

    assert_eq!(remainder, "");
    assert_eq!(result, (1f64, "d", "..", 7f64, "d"))
}
