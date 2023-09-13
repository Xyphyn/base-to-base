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

    #[arg(value_enum, default_value_t=Base::Dec)]
    base: Base,
}

fn format_line(prefix: &str, number: String) -> String {
    format!("\x1b[37;1m{}: \x1b[0m{}", prefix, number)
}

fn main() {
    let args = Args::parse();

    let number = match parse_number(&args.number, args.base) {
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
