use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn fdiff(fp_one: &str, fp_two: &str) {
    let lines_two = readlines(fp_two);

    for line in readlines(fp_one) {
        if ! lines_two.contains(&line) {
            println!("{}", line);
        }
    }
}

fn help(name: &str) {
    println!("Usage: {} one.txt two.txt", name);
    println!("Prints lines that exist in one.txt but not in two.txt");
}

fn readlines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("failed to parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let bin_name = args.get(0).unwrap();
    
    let f_one = match args.get(1) {
        Some(file) => file,
        None => {
            help(bin_name);
            return
        }
    };

    let f_two = match args.get(2) {
        Some(file) => file,
        None => {
            help(bin_name);
            return
        }
    };

    fdiff(f_one, f_two);
}
