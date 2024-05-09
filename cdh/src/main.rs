///! Convert a single DEC string to HEX
use std::{env, i64};

fn convert(str_dec: &str) {
    match i64::from_str_radix(str_dec, 10) {
        Ok(out) => println!("{:#04x}", out),
        Err(..) => println!("Conversion failed!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let str_dec = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No argument!");
            return
        }
    };

    convert(str_dec);
}

