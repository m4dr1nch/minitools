///! Decode a URL encoded string
use std::env;
use std::io;
use percent_encoding::percent_decode_str;

fn decode(str_dec: &str) {
    match percent_decode_str(str_dec).decode_utf8() {
        Ok(out) => print!("{}", out.trim_end()),
        Err(..) => println!("Decode failed!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1) {
        Some(arg) => {
            decode(arg);
        },
        None => {
            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(len) => if len == 0 {
                        return;
                    } else {
                        decode(&input);
                    }
                    Err(..) => {
                        println!("Failed reading STDIN.");
                        return
                    }
                }
            }
        }
    };
}

