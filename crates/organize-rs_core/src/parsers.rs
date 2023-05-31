//! parser

use std::{fmt::Display, ops::Range, str::FromStr};

use serde::{Deserialize, Serialize};
use winnow::{
    ascii::alpha0,
    combinator::alt,
    error::{Error, ErrorKind},
    token::take_while,
    IResult, Parser,
};

use byte_unit::Byte;

use crate::filters::DateUnitKind;

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PeriodRange(Range<f64>);

impl PeriodRange {
    pub const MIN: f64 = 0f64;
    pub const MAX: f64 = 9.467e+8f64; // 30 years in seconds

    pub fn in_range(&self, value: f64) -> bool {
        self.0.contains(&value)
    }
}

impl Display for PeriodRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PeriodRange({:?})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SizeRange(Range<f64>);

impl Display for SizeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SizeRange({:?})", self.0)
    }
}
impl SizeRange {
    pub const MIN: f64 = 1f64; // 1 byte
    pub const MAX: f64 = 4e+12f64; // 4 TB in bytes

    pub fn in_range(&self, size: f64) -> bool {
        self.0.contains(&size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundarySide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleCondition<T> {
    value: T,
    side: BoundarySide,
}

#[derive(Debug, Clone, Copy)]
pub struct Condition<T>(Option<SingleCondition<T>>, Option<SingleCondition<T>>);

impl<T> Condition<T> {
    pub fn set_condition(&mut self, condition: SingleCondition<T>) {
        match condition.side {
            BoundarySide::Left => self.0 = Some(condition),
            BoundarySide::Right => self.1 = Some(condition),
        }
    }
}

impl FromStr for PeriodRange {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut condition = Condition(None, None);

        match (
            parse_left_boundary(input),
            parse_right_boundary(input),
            parse_whole_condition(input),
        ) {
            (Ok(left_boundary), Err(_), Err(_)) => {
                let (_, (value, unit, _)) = left_boundary;

                condition.set_condition(SingleCondition {
                    value: DateUnitKind::from((value, unit)).into_seconds(),
                    side: BoundarySide::Left,
                });
            }
            (Err(_), Ok(right_boundary), Err(_)) => {
                let (_, (_, value, unit)) = right_boundary;

                condition.set_condition(SingleCondition {
                    value: DateUnitKind::from((value, unit)).into_seconds(),
                    side: BoundarySide::Right,
                });
            }
            (_, _, Ok(whole_condition)) => {
                let (_, (value_left, unit_left, _, value_right, unit_right)) = whole_condition;

                if unit_left != unit_right {
                    return Err(Error {
                        input: format!("Left unit {unit_left} and right unit {unit_right} need to be equal: {input}"),
                        kind: ErrorKind::Verify,
                    });
                } else if value_left >= value_right {
                    return Err(Error {
                        input: format!("Left value {value_left} can't be greater or equal than the right value {value_right}: {input}"),
                        kind: ErrorKind::Verify,
                    });
                }

                condition.set_condition(SingleCondition {
                    value: DateUnitKind::from((value_left, unit_left)).into_seconds(),
                    side: BoundarySide::Left,
                });

                condition.set_condition(SingleCondition {
                    value: DateUnitKind::from((value_right, unit_right)).into_seconds(),
                    side: BoundarySide::Right,
                });
            }
            _ => {
                return Err(Error {
                    input: format!("Size condition pattern not recognized for: {input}"),
                    kind: ErrorKind::Verify,
                })
            }
        }

        match condition {
            Condition(Some(left), None) => Ok(Self(Range {
                start: left.value,
                end: PeriodRange::MAX,
            })),
            Condition(None, Some(right)) => Ok(Self(Range {
                start: PeriodRange::MIN,
                end: right.value,
            })),
            Condition(Some(left), Some(right)) if left.value < right.value => Ok(Self(Range {
                start: left.value,
                end: right.value,
            })),
            _ => unreachable!("shouldn't be able to create a condition!"),
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
                    value: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
                    side: BoundarySide::Left,
                });
            }
            (Err(_), Ok(right_boundary), Err(_)) => {
                let (_, (_, size, unit)) = right_boundary;

                condition.set_condition(SingleCondition {
                    value: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
                    side: BoundarySide::Right,
                });
            }
            (_, _, Ok(whole_condition)) => {
                let (_, (size_left, unit_left, _, size_right, unit_right)) = whole_condition;

                condition.set_condition(SingleCondition {
                    value: Byte::from_str(format!("{size_left} {unit_left}")).map_err(|err| {
                        Error {
                            input: format!(
                                "Couldn't convert {size_left} {unit_left} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        }
                    })?,
                    side: BoundarySide::Left,
                });

                condition.set_condition(SingleCondition {
                    value: Byte::from_str(format!("{size_right} {unit_right}")).map_err(|err| {
                        Error {
                            input: format!(
                                "Couldn't convert {size_right} {unit_right} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        }
                    })?,
                    side: BoundarySide::Right,
                });
            }
            _ => {
                return Err(Error {
                    input: format!("Size condition pattern not recognized for: {input}"),
                    kind: ErrorKind::Verify,
                })
            }
        }

        match condition {
            Condition(Some(left), None) => Ok(Self(Range {
                start: left.value.get_bytes() as f64,
                end: SizeRange::MAX,
            })),
            Condition(None, Some(right)) => Ok(Self(Range {
                start: SizeRange::MIN,
                end: right.value.get_bytes() as f64,
            })),
            Condition(Some(left), Some(right))
                if left.value.get_bytes() < right.value.get_bytes() =>
            {
                Ok(Self(Range {
                    start: left.value.get_bytes() as f64,
                    end: right.value.get_bytes() as f64,
                }))
            }
            Condition(Some(left), Some(right))
                if left.value.get_bytes() >= right.value.get_bytes() =>
            {
                Err(Error {
                    input: format!(
                        "Left value {} can't be greater or equal than the right value {}: {input}",
                        left.value.get_bytes(),
                        right.value.get_bytes()
                    ),
                    kind: ErrorKind::Verify,
                })
            }
            _ => unreachable!("shouldn't be able to create a condition!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_left_size_condition_to_range_passes() {
        let condition = "5.0GiB..";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            5368709120.0..4000000000000.0,
        )
        "###);
    }

    #[test]
    fn parse_right_size_condition_to_range_passes() {
        let condition = "..0.5GiB";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            1.0..536870912.0,
        )
        "###);
    }

    #[test]
    fn parse_whole_size_condition_to_range_passes() {
        let condition = "1.5MiB..100.3MB";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            1572864.0..100300000.0,
        )
        "###);
    }

    #[test]
    fn parse_whole_period_condition_to_range_passes() {
        let condition = "1d..7d";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        PeriodRange(
            86400.0..604800.0,
        )
        "###);
    }

    #[test]
    fn parse_left_period_condition_to_range_passes() {
        let condition = "1d..";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        PeriodRange(
            86400.0..946700000.0,
        )
        "###);
    }

    #[test]
    fn parse_right_period_condition_to_range_passes() {
        let condition = "..1d";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        PeriodRange(
            0.0..86400.0,
        )
        "###);
    }

    #[test]
    #[should_panic]
    fn parse_different_units_on_whole_period_condition_to_range_fails() {
        let condition = "1w..7d";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r"");
    }

    #[test]
    #[should_panic]
    fn parse_non_standard_units_on_whole_period_condition_to_range_fails() {
        let condition = "1f..7f";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r"");
    }
}
