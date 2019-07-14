#![warn(rust_2018_idioms)]

use pretty_assertions::assert_eq; //, assert_ne};

use nom;
use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub width: u32,
    pub height: u32,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_header_start(input: &str) -> IResult<&str, Header> {
    let (input, _) = tag("P")(input)?;
    let (input, _) = take_while(|c| "36".contains(c))(input)?;
    let (input, _) = take_while(|c| " \n\r".contains(c))(input)?;
    let (input, width) = take_while(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = take_while(|c| " \n\r".contains(c))(input)?;
    let (input, height) = take_while(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = take_while(|c| " \n\r".contains(c))(input)?;

    Ok((
        input,
        Header {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
        },
    ))
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn parse_hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn main() {
    let bytes = include_bytes!("boxes_1.ppm");
    //nom::char();
    println!("{:#?}", bytes[0]);
}

#[test]
fn parse_color() {
    assert_eq!(
        parse_hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}

#[test]
fn parse_header() {
    assert_eq!(
        parse_header_start("P6 32 32"),
        Ok((
            "",
            Header {
                width: 32,
                height: 32,
            }
        ))
    );
}

#[test]
fn parse_header2() {
    assert_eq!(
        parse_header_start("P6 109 23"),
        Ok((
            "",
            Header {
                width: 109,
                height: 23,
            }
        ))
    );
}
