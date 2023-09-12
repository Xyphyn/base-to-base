extern crate clap;
use std::{convert::TryInto, num};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Base {
    #[clap(alias("x"))]
    Hex,
    #[clap(alias("d"))]
    Dec,
    #[clap(alias("o"))]
    Oct,
    #[clap(alias("b"))]
    Bin,
}

impl std::str::FromStr for Base {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x" => Ok(Base::Hex),
            "hex" => Ok(Base::Hex),
            "d" => Ok(Base::Dec),
            "dec" => Ok(Base::Dec),
            "o" => Ok(Base::Oct),
            "oct" => Ok(Base::Oct),
            "b" => Ok(Base::Bin),
            "bin" => Ok(Base::Bin),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

pub fn parse_number(number: &String, base: Base) -> Result<i128, num::ParseIntError> {
    return match base {
        Base::Hex => i128::from_str_radix(number, 16),
        Base::Dec => i128::from_str_radix(number, 10),
        Base::Oct => i128::from_str_radix(number, 8),
        Base::Bin => i128::from_str_radix(number, 2),
    };
}

pub fn format_radix(mut x: i128, radix: i128) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m.try_into().unwrap(), radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

pub fn base_to_str(number: i128, out_base: Base) -> String {
    return format_radix(
        number,
        match out_base {
            Base::Hex => 16,
            Base::Dec => 10,
            Base::Oct => 8,
            Base::Bin => 2,
        },
    );
}

#[cfg(test)]
mod tests {
    use crate::{base_to_str, format_radix, parse_number, Base};

    #[test]
    fn numbers_parse() {
        assert_eq!(parse_number(&String::from("f"), Base::Hex), Ok(15));
        assert_eq!(parse_number(&String::from("15"), Base::Dec), Ok(15));
        assert_eq!(parse_number(&String::from("17"), Base::Oct), Ok(15));
        assert_eq!(parse_number(&String::from("1111"), Base::Bin), Ok(15));
    }

    #[test]
    fn bases_convert() {
        assert_eq!(base_to_str(15, Base::Hex), "f");
        assert_eq!(base_to_str(15, Base::Dec), "15");
        assert_eq!(base_to_str(15, Base::Oct), "17");
        assert_eq!(base_to_str(15, Base::Bin), "1111");
    }

    #[test]
    fn radix_formats() {
        assert_eq!(format_radix(15, 16), "f")
    }
}
