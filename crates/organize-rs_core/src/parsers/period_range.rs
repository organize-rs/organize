//! parser for [`PeriodRange`]

use std::{fmt::Display, ops::Range, str::FromStr};

use serde::Serialize;
use serde_with::DeserializeFromStr;
use winnow::{
    error::{Error, ErrorKind},
    Parser,
};

use crate::{
    filters::DateUnitKind,
    parsers::{
        parse_left_range_boundary, parse_right_range_boundary, parse_whole_range, Condition,
        RangeBoundarySide, SingleRangeCondition,
    },
};

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, Serialize)]
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

impl FromStr for PeriodRange {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut condition = Condition(None, None);

        match (
            parse_whole_range(input),
            parse_left_range_boundary(input),
            parse_right_range_boundary(input),
        ) {
            (Ok(whole_condition), _, _) => {
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

                condition.set_condition(SingleRangeCondition {
                    value: DateUnitKind::from((value_left, unit_left)).into_seconds(),
                    side: RangeBoundarySide::Left,
                });

                condition.set_condition(SingleRangeCondition {
                    value: DateUnitKind::from((value_right, unit_right)).into_seconds(),
                    side: RangeBoundarySide::Right,
                });
            }
            (_, Ok(left_boundary), _) => {
                let (_, (value, unit, _)) = left_boundary;

                condition.set_condition(SingleRangeCondition {
                    value: DateUnitKind::from((value, unit)).into_seconds(),
                    side: RangeBoundarySide::Left,
                });
            }
            (_, _, Ok(right_boundary)) => {
                let (_, (_, value, unit)) = right_boundary;

                condition.set_condition(SingleRangeCondition {
                    value: DateUnitKind::from((value, unit)).into_seconds(),
                    side: RangeBoundarySide::Right,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_whole_period_condition_to_range_passes() {
        let condition = "1d..7d";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        PeriodRange(
            86400.0..604800.0,
        )
        "###);
    }

    #[test]
    fn test_parse_left_period_condition_to_range_passes() {
        let condition = "1d..";
        let range = PeriodRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        PeriodRange(
            86400.0..946700000.0,
        )
        "###);
    }

    #[test]
    fn test_parse_right_period_condition_to_range_passes() {
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
    fn test_parse_different_units_on_whole_period_condition_to_range_fails() {
        let condition = "1w..7d";
        let _range = PeriodRange::from_str(condition).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_non_standard_units_on_whole_period_condition_to_range_fails() {
        let condition = "1f..7f";
        let _range = PeriodRange::from_str(condition).unwrap();
    }
}
