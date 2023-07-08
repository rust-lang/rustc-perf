//! nom 7 JSON parser taken from https://github.com/rust-bakery/nom/blob/90d78d65a10821272ce8856570605b07a917a6c1/tests/json.rs.
use std::collections::HashMap;

use nom::multi::fold_many0;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char, multispace0, none_of},
    combinator::{map, map_opt, map_res, value, verify},
    error::ParseError,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(false, tag("false")), value(true, tag("true")))).parse(input)
}

fn u16_hex(input: &str) -> IResult<&str, u16> {
    map_res(take(4usize), |s| u16::from_str_radix(s, 16)).parse(input)
}

fn unicode_escape(input: &str) -> IResult<&str, char> {
    map_opt(
        alt((
            // Not a surrogate
            map(verify(u16_hex, |cp| !(0xD800..0xE000).contains(cp)), |cp| {
                cp as u32
            }),
            // See https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF for details
            map(
                verify(
                    separated_pair(u16_hex, tag("\\u"), u16_hex),
                    |(high, low)| (0xD800..0xDC00).contains(high) && (0xDC00..0xE000).contains(low),
                ),
                |(high, low)| {
                    let high_ten = (high as u32) - 0xD800;
                    let low_ten = (low as u32) - 0xDC00;
                    (high_ten << 10) + low_ten + 0x10000
                },
            ),
        )),
        // Could be probably replaced with .unwrap() or _unchecked due to the verify checks
        std::char::from_u32,
    )
    .parse(input)
}

fn character(input: &str) -> IResult<&str, char> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((
            map_res(anychar, |c| {
                Ok(match c {
                    '"' | '\\' | '/' => c,
                    'b' => '\x08',
                    'f' => '\x0C',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err(()),
                })
            }),
            preceded(char('u'), unicode_escape),
        ))
        .parse(input)
    } else {
        Ok((input, c))
    }
}

fn string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        fold_many0(character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        char('"'),
    )
    .parse(input)
}

fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl Parser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

fn array(input: &str) -> IResult<&str, Vec<JsonValue>> {
    delimited(
        char('['),
        ws(separated_list0(ws(char(',')), json_value)),
        char(']'),
    )
    .parse(input)
}

fn object(input: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    map(
        delimited(
            char('{'),
            ws(separated_list0(
                ws(char(',')),
                separated_pair(string, ws(char(':')), json_value),
            )),
            char('}'),
        ),
        |key_values| key_values.into_iter().collect(),
    )
    .parse(input)
}

fn json_value(input: &str) -> IResult<&str, JsonValue> {
    use JsonValue::*;

    alt((
        value(Null, tag("null")),
        map(boolean, Bool),
        map(string, Str),
        map(double, Num),
        map(array, Array),
        map(object, Object),
    ))
    .parse(input)
}

pub fn parse_json(input: &str) -> IResult<&str, JsonValue> {
    ws(json_value).parse(input)
}
