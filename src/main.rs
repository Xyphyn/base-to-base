use core::num;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short('x'))]
    hex: bool,

    #[arg(long, short)]
    dec: bool,

    #[arg(long, short)]
    oct: bool,

    #[arg(long, short)]
    bin: bool,

    #[arg(trailing_var_arg(true))]
    number: Vec<String>,
}

enum Base {
    Hex,
    Dec,
    Oct,
    Bin,
}

fn parse_number(number: &String, base: Base) -> Result<i128, num::ParseIntError> {
    return match base {
        Base::Hex => i128::from_str_radix(number, 16),
        Base::Dec => i128::from_str_radix(number, 10),
        Base::Oct => i128::from_str_radix(number, 8),
        Base::Bin => i128::from_str_radix(number, 2),
    };
}

fn bools_to_base(bools: (bool, bool, bool, bool)) -> Base {
    return match bools {
        (true, _, _, _) => Base::Hex,
        (_, true, _, _) => Base::Dec,
        (_, _, true, _) => Base::Oct,
        (_, _, _, true) => Base::Bin,
        _ => Base::Dec,
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

    if args.number.len() != 1 {
        println!("Invalid arg count");
        return;
    }

    let number_str = args.number.get(0);
    if number_str.is_none() {
        println!("Invalid number");
        return;
    }

    let number_str = number_str.unwrap();

    let number = parse_number(
        number_str,
        bools_to_base((args.hex, args.dec, args.oct, args.bin)),
    );

    if number.is_err() {
        println!("Invalid number");
        return;
    }

    let number = number.unwrap();

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
}
