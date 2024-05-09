///! Convert a single HEX string to DEC
use std::{env, i64};

fn convert(str_hex: &str) {
    match i64::from_str_radix(str_hex, 16) {
        Ok(out) => println!("{}", out),
        Err(..) => println!("Conversion failed!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let str_hex = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No argument!");
            return
        }
    };
    let prefixes = ["0x", "\\x"];

    for prefix in prefixes {
        match str_hex.strip_prefix(prefix) {
            Some(hex) => {
                convert(hex);
                return
            },
            None => continue
        };
    }

    println!("Invalid prefix!");
}
