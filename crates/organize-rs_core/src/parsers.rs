//! parsers

pub mod period_range;
pub mod size_range;
pub mod template;

use winnow::{
    ascii::{alpha0, float},
    combinator::alt,
    IResult, Parser,
};

pub fn parse_range_boundary(input: &str) -> IResult<&str, &str> {
    alt((("..="), (".."))).parse_next(input)
}

pub fn parse_digits(input: &str) -> IResult<&str, f64> {
    float(input)
}

pub fn parse_right_range_boundary(input: &str) -> IResult<&str, (&str, f64, &str)> {
    (parse_range_boundary, parse_digits, parse_units).parse_next(input)
}

pub fn parse_left_range_boundary(input: &str) -> IResult<&str, (f64, &str, &str)> {
    (parse_digits, parse_units, parse_range_boundary).parse_next(input)
}

pub fn parse_units(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

pub fn parse_whole_range(input: &str) -> IResult<&str, (f64, &str, &str, f64, &str)> {
    (
        parse_digits,
        parse_units,
        parse_range_boundary,
        parse_digits,
        parse_units,
    )
        .parse_next(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeBoundarySide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleRangeCondition<T> {
    value: T,
    side: RangeBoundarySide,
}

#[derive(Debug, Clone, Copy)]
pub struct Condition<T>(
    Option<SingleRangeCondition<T>>,
    Option<SingleRangeCondition<T>>,
);

impl<T> Condition<T> {
    pub fn set_condition(&mut self, condition: SingleRangeCondition<T>) {
        match condition.side {
            RangeBoundarySide::Left => self.0 = Some(condition),
            RangeBoundarySide::Right => self.1 = Some(condition),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[should_panic]
    #[case("1d..7d", (1.0, "d", ".."))]
    #[case("1w..", (1.0, "w", "..") )]
    fn test_parse_left_range_boundary_passes(
        #[case] condition: &str,
        #[case] outcome: (f64, &str, &str),
    ) {
        let (remainder, result) = (parse_left_range_boundary).parse_next(condition).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }

    #[rstest]
    #[should_panic]
    #[case("1d..7d", ("..", 7.0, "d"))]
    #[case("..1w", ("..", 1.0, "w"))]
    fn test_parse_right_range_boundary_passes(
        #[case] condition: &str,
        #[case] outcome: (&str, f64, &str),
    ) {
        let (remainder, result) = (parse_right_range_boundary).parse_next(condition).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }

    #[rstest]
    #[should_panic]
    #[case("1w..", (1.0, "w", "..", 0.0, "") )]
    #[case("1d..7d", (1.0, "d", "..", 7.0, "d"))]
    fn test_parse_whole_range_passes(
        #[case] condition: &str,
        #[case] outcome: (f64, &str, &str, f64, &str),
    ) {
        let (remainder, result) = (parse_whole_range).parse_next(condition).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }
}
