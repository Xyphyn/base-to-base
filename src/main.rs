use btb::{base_to_str, parse_number, Base};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, short, long)]
    to: Option<Base>,

    number: Option<String>,

    #[arg(value_enum, default_value_t=Base::Dec)]
    base: Base,
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
