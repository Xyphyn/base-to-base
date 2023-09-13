extern crate btb;
extern crate clap;

use std::process;

use btb::{base_to_str, parse_number, Base};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, short, long)]
    to: Option<Base>,

    number: String,

    #[arg(value_enum)]
    base: Option<Base>,
}

fn format_line(prefix: &str, number: String) -> String {
    format!("\x1b[37;1m{}: \x1b[0m{}", prefix, number)
}

fn get_str_base(string: String) -> (Base, String) {
    let string = string.to_lowercase();

    if let Some(stripped) = string.strip_prefix("0x") {
        return (Base::Hex, stripped.to_string());
    }

    if let Some(stripped) = string.strip_prefix("0b") {
        return (Base::Bin, stripped.to_string());
    }

    if let Some(stripped) = string.strip_prefix('0') {
        return (Base::Oct, stripped.to_string());
    }

    (Base::Dec, string)
}

fn main() {
    let args = Args::parse();

    let str_base = get_str_base(String::from(&args.number));

    let base = match args.base {
        Some(base) => base,
        None => str_base.0,
    };

    let number = match parse_number(&str_base.1, base) {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid number for base");
            process::exit(1);
        }
    };

    if args.to.is_none() {
        println!(
            "{}
{}
{}
{}
        ",
            format_line("Hex", base_to_str(number, Base::Hex)),
            format_line("Dec", base_to_str(number, Base::Dec)),
            format_line("Oct", base_to_str(number, Base::Oct)),
            format_line("Bin", base_to_str(number, Base::Bin)),
        )
    } else {
        let to = args.to.unwrap();

        println!("{}", base_to_str(number, to))
    }
}
