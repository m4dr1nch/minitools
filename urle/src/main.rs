///! Encode a URL encoded string
use std::env;
use std::io;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

fn encode(str_enc: &str) {
    let res = utf8_percent_encode(str_enc, NON_ALPHANUMERIC).to_string();
    print!("{}", res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1) {
        Some(arg) => {
            encode(arg);
        },
        None => {
            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(len) => if len == 0 {
                        return;
                    } else {
                        encode(&input);
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
