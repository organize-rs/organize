//! parsers

pub mod period_range;
pub mod size_range;

use std::{fmt::Display, ops::Range, str::FromStr};

use serde::Serialize;
use serde_with::DeserializeFromStr;
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
