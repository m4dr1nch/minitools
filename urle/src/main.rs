///! Encode a URL encoded string
use std::env;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

fn encode(str_enc: &str) {
    let res = utf8_percent_encode(str_enc, NON_ALPHANUMERIC).to_string();
    println!("{}", res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let str_to_url = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No argument!");
            return
        }
    };

    encode(str_to_url);
}
