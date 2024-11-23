///! Decode a URL encoded string
use std::env;
use percent_encoding::percent_decode_str;

fn decode(str_dec: &str) {
    match percent_decode_str(str_dec).decode_utf8() {
        Ok(out) => println!("{}", out),
        Err(..) => println!("Decode failed!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let str_url = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No argument!");
            return
        }
    };

    decode(str_url);
}

