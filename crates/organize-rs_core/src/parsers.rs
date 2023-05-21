use std::{ops::Range, str::FromStr};

use winnow::{
    ascii::alpha0,
    combinator::alt,
    error::{Error, ErrorKind},
    token::take_while,
    IResult, Parser,
};

use byte_unit::Byte;

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

pub fn parse_units(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SizeRange(Range<f64>);

#[derive(Debug, Clone, Copy)]
pub enum BoundarySide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct SingleCondition {
    byte_size: Byte,
    boundary: BoundarySide,
}

#[derive(Debug, Clone, Copy)]
pub struct Condition(Option<SingleCondition>, Option<SingleCondition>);

impl Condition {
    pub fn set_condition(&mut self, condition: SingleCondition) {
        match condition.boundary {
            BoundarySide::Left => self.0 = Some(condition),
            BoundarySide::Right => self.1 = Some(condition),
        }
    }
}

impl FromStr for SizeRange {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut condition = Condition(None, None);

        match (
            parse_left_boundary(input),
            parse_right_boundary(input),
            parse_whole_condition(input),
        ) {
            (Ok(left_boundary), Err(_), Err(_)) => {
                let (_, (size, unit, _)) = left_boundary;

                condition.set_condition(SingleCondition {
                    byte_size: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
                    boundary: BoundarySide::Left,
                });
            }
            (Err(_), Ok(right_boundary), Err(_)) => {
                let (_, (_, size, unit)) = right_boundary;

                condition.set_condition(SingleCondition {
                    byte_size: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
                    boundary: BoundarySide::Right,
                });
            }
            (_, _, Ok(whole_condition)) => {
                let (_, (size_left, unit_left, _, size_right, unit_right)) = whole_condition;

                condition.set_condition(SingleCondition {
                    byte_size: Byte::from_str(format!("{size_left} {unit_left}")).map_err(
                        |err| Error {
                            input: format!(
                                "Couldn't convert {size_left} {unit_left} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        },
                    )?,
                    boundary: BoundarySide::Left,
                });

                condition.set_condition(SingleCondition {
                    byte_size: Byte::from_str(format!("{size_right} {unit_right}")).map_err(
                        |err| Error {
                            input: format!(
                                "Couldn't convert {size_right} {unit_right} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        },
                    )?,
                    boundary: BoundarySide::Right,
                });
            }
            _ => {
                return Err(Error {
                    input: format!("Size condition pattern not recognized for: {input}"),
                    kind: ErrorKind::Fail,
                })
            }
        }

        match condition {
            Condition(Some(left), None) => Ok(Self(Range {
                start: left.byte_size.get_bytes() as f64,
                end: f64::MAX,
            })),
            Condition(None, Some(right)) => Ok(Self(Range {
                start: 0f64,
                end: right.byte_size.get_bytes() as f64,
            })),
            Condition(Some(left), Some(right)) => Ok(Self(Range {
                start: left.byte_size.get_bytes() as f64,
                end: right.byte_size.get_bytes() as f64,
            })),
            _ => unreachable!("shouldn't be able to create a condition!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::ops::Range;

    #[test]
    fn parse_left_size_condition_to_range_passes() {
        let condition = "5.0GiB..";
        let range = SizeRange::from_str(condition).unwrap();

        assert_eq!(
            range.0,
            Range {
                start: 5f64 * 1_024f64 * 1_024f64 * 1_024f64,
                end: f64::MAX
            }
        );
    }

    #[test]
    fn parse_right_size_condition_to_range_passes() {
        let condition = "..0.5GiB";
        let range = SizeRange::from_str(condition).unwrap();
        assert_eq!(
            range.0,
            Range {
                start: 0f64,
                end: 0.5f64 * 1024f64 * 1_024f64 * 1_024f64
            }
        );
    }

    #[test]
    fn parse_whole_size_condition_to_range_passes() {
        let condition = "1.5MiB..100.3MB";
        let range = SizeRange::from_str(condition).unwrap();
        assert_eq!(
            range.0,
            Range {
                start: 1.5f64 * 1_024f64 * 1_024f64,
                end: 100.3f64 * 1_000f64 * 1_000f64
            }
        );
    }
}
