//! parsers

pub mod period_range;
pub mod size_range;
pub mod template;

use winnow::{
    ascii::{alpha0, float},
    combinator::alt,
    IResult, Parser,
};

/// Parses a range boundary, which is either ".." or "..=".
/// The first is exclusive, the second is inclusive.
///
/// # Arguments
///
/// * `input` - The input string to parse.
pub fn parse_range_boundary(input: &str) -> IResult<&str, &str> {
    alt((("..="), (".."))).parse_next(input)
}

/// Parses a number of floating point digits.
///
/// # Arguments
///
/// * `input` - The input string to parse.
pub fn parse_digits(input: &str) -> IResult<&str, f64> {
    float(input)
}

/// Parses a right range boundary, which is a range boundary followed by a number of digits.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// A tuple of the range boundary, the floating point value, and the unit.
pub fn parse_right_range_boundary(input: &str) -> IResult<&str, (&str, f64, &str)> {
    (parse_range_boundary, parse_digits, parse_units).parse_next(input)
}

/// Parses a left range boundary, which is a number of digits followed by a range boundary.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// A tuple of the floating point value, the unit, and the range boundary.
pub fn parse_left_range_boundary(input: &str) -> IResult<&str, (f64, &str, &str)> {
    (parse_digits, parse_units, parse_range_boundary).parse_next(input)
}

/// Parses a unit, which is a string of alphabetic characters.
///
/// # Arguments
///
/// * `input` - The input string to parse.
pub fn parse_units(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

/// Parses a whole range, which is digits followed by a unit, followed by a range boundary,
/// followed by digits followed by a unit.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// A tuple of the left value, the left unit, the range boundary, the right value, and the right unit.
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

/// Which side of a range boundary a value is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeBoundarySide {
    /// The value is on the left side of the range boundary.
    Left,
    /// The value is on the right side of the range boundary.
    Right,
}

/// A single range condition, which is a value and a side of a range boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleRangeCondition<T> {
    /// The value of the condition.
    value: T,
    /// The side of the range boundary the value is on.
    side: RangeBoundarySide,
}

/// A condition, which is a pair of single range conditions.
#[derive(Debug, Clone, Copy)]
pub struct Condition<T>(
    Option<SingleRangeCondition<T>>,
    Option<SingleRangeCondition<T>>,
);

impl<T> Condition<T> {
    /// Set a single range condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to set.
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
