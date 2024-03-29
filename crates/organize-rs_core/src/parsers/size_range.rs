//! parser for [`SizeRange`]

use std::{fmt::Display, ops::Range, str::FromStr};

use serde::Serialize;
use serde_with::DeserializeFromStr;
use winnow::{
    error::{Error, ErrorKind},
    Parser,
};

use byte_unit::Byte;

use crate::parsers::{
    parse_left_range_boundary, parse_right_range_boundary, parse_whole_range, Condition,
    RangeBoundarySide, SingleRangeCondition,
};

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, Serialize)]
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

impl FromStr for SizeRange {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut condition = Condition(None, None);

        match (
            parse_whole_range(input),
            parse_left_range_boundary(input),
            parse_right_range_boundary(input),
        ) {
            (Ok(whole_condition), _, _) => {
                let (_, (size_left, unit_left, _, size_right, unit_right)) = whole_condition;

                condition.set_condition(SingleRangeCondition {
                    value: Byte::from_str(format!("{size_left} {unit_left}")).map_err(|err| {
                        Error {
                            input: format!(
                                "Couldn't convert {size_left} {unit_left} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        }
                    })?,
                    side: RangeBoundarySide::Left,
                });

                condition.set_condition(SingleRangeCondition {
                    value: Byte::from_str(format!("{size_right} {unit_right}")).map_err(|err| {
                        Error {
                            input: format!(
                                "Couldn't convert {size_right} {unit_right} to bytes: {err}"
                            ),
                            kind: ErrorKind::Fail,
                        }
                    })?,
                    side: RangeBoundarySide::Right,
                });
            }
            (_, Ok(left_boundary), _) => {
                let (_, (size, unit, _)) = left_boundary;

                condition.set_condition(SingleRangeCondition {
                    value: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
                    side: RangeBoundarySide::Left,
                });
            }
            (_, _, Ok(right_boundary)) => {
                let (_, (_, size, unit)) = right_boundary;

                condition.set_condition(SingleRangeCondition {
                    value: Byte::from_str(format!("{size} {unit}")).map_err(|err| Error {
                        input: format!("Couldn't convert {size} {unit} to bytes: {err}"),
                        kind: ErrorKind::Fail,
                    })?,
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
    fn test_parse_left_size_condition_to_range_passes() {
        let condition = "5.0GiB..";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            5368709120.0..4000000000000.0,
        )
        "###);
    }

    #[test]
    fn test_parse_right_size_condition_to_range_passes() {
        let condition = "..0.5GiB";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            1.0..536870912.0,
        )
        "###);
    }

    #[test]
    fn test_parse_whole_size_condition_to_range_passes() {
        let condition = "1.5MiB..100.3MB";
        let range = SizeRange::from_str(condition).unwrap();
        insta::assert_debug_snapshot!(range, @r###"
        SizeRange(
            1572864.0..100300000.0,
        )
        "###);
    }
}
