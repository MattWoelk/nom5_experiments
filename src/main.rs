#![warn(rust_2018_idioms)]

use pretty_assertions::assert_eq; //, assert_ne};

use nom;
use nom::sequence::terminated;
use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PPMDataType {
    ASCII,
    BYTE,
}

type PPMData = Vec<Vec<Color>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    pub data_type: PPMDataType,
    pub width: u32,
    pub height: u32,
    pub max_value: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PPM {
    pub header: Header,
    pub data: PPMData,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

//fn consume_whitespace<Input>(input: &str) -> IResult<&str, &str> {
//    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
//    Ok((input, input))
//}

fn is_whitespace(c: char) -> bool {
    " \n\r\t".contains(c)
}

fn parse_ppm_header(input: &str) -> IResult<&str, Header> {
    let take_alphanumeric = take_while(char::is_alphanumeric);
    let take_whitespace = take_while(is_whitespace);

    let (input, _) = tag("P")(input)?;
    let (input, data_type) = terminated(take_while(|c| "36".contains(c)), &take_whitespace)(input)?;
    let (input, width) = take_alphanumeric(input)?;
    let (input, _) = take_whitespace(input)?;
    let (input, height) = take_alphanumeric(input)?;
    let (input, _) = take_whitespace(input)?;
    let (input, max_value) = take_alphanumeric(input)?;
    let (input, _) = take_whitespace(input)?;

    Ok((
        input,
        Header {
            data_type: match data_type {
                "3" => PPMDataType::ASCII,
                _ => PPMDataType::BYTE,
            },
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            max_value: max_value.parse().unwrap(),
        },
    ))
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn parse_ppm_data_ascii(input: &str) -> IResult<&str, PPMData> {
    unimplemented!();
}

fn parse_ppm_data_bytes(input: &str) -> IResult<&str, PPMData> {
    unimplemented!();
}

fn parse_ppm(input: &str) -> IResult<&str, PPM> {
    let (input, header) = parse_ppm_header(input)?;

    Ok((
        "",
        PPM {
            header: header.clone(),
            data: match &header.data_type {
                PPMDataType::ASCII => parse_ppm_data_ascii(input)?.1,
                PPMDataType::BYTE => parse_ppm_data_bytes(input)?.1,
            },
        },
    ))
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
        parse_ppm_header("P3 32 32 255"),
        Ok((
            "",
            Header {
                data_type: PPMDataType::ASCII,
                width: 32,
                height: 32,
                max_value: 255,
            }
        ))
    );
}

#[test]
fn parse_header2() {
    assert_eq!(
        parse_ppm_header("P6 109 23 10"),
        Ok((
            "",
            Header {
                data_type: PPMDataType::BYTE,
                width: 109,
                height: 23,
                max_value: 10,
            }
        ))
    );
}

#[test]
fn parse_ascii1() {
    assert_eq!(
        parse_ppm(
            "P3
2 2
15
 0  0  0    0  0  0
 0  0  0    0 15  7"
        ),
        Ok((
            "",
            PPM {
                header: Header {
                    data_type: PPMDataType::ASCII,
                    width: 2,
                    height: 2,
                    max_value: 15,
                },
                data: vec![
                    vec![
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0
                        },
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0
                        }
                    ],
                    vec![
                        Color {
                            red: 0,
                            green: 0,
                            blue: 0
                        },
                        Color {
                            red: 0,
                            green: 255,
                            blue: 119
                        }
                    ]
                ],
            }
        ))
    );
}
