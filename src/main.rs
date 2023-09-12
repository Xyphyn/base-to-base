use core::num;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Base {
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, short, long)]
    to: Option<Base>,

    number: Option<String>,

    #[arg(value_enum, default_value_t=Base::Dec)]
    base: Base,
}

fn parse_number(number: &String, base: Base) -> Result<i128, num::ParseIntError> {
    return match base {
        Base::Hex => i128::from_str_radix(number, 16),
        Base::Dec => i128::from_str_radix(number, 10),
        Base::Oct => i128::from_str_radix(number, 8),
        Base::Bin => i128::from_str_radix(number, 2),
    };
}

fn format_radix(mut x: i128, radix: i128) -> String {
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

fn base_to_str(number: i128, out_base: Base) -> String {
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

fn main() {
    let args = Args::parse();

    let number_str = args.number.unwrap();

    let number = parse_number(&number_str, args.base);

    if number.is_err() {
        println!("Invalid number for base");
        return;
    }

    let number = number.unwrap();

    if args.to.is_none() {
        println!(
            "\x1b[37;1mHex: \x1b[0m{}
\x1b[37;1mDec: \x1b[0m{}
\x1b[37;1mOct: \x1b[0m{}
\x1b[37;1mBin: \x1b[0m{}
        ",
            base_to_str(number, Base::Hex),
            base_to_str(number, Base::Dec),
            base_to_str(number, Base::Oct),
            base_to_str(number, Base::Bin),
        )
    } else {
        let to = args.to.unwrap();

        println!("{}", base_to_str(number, to))
    }
}
