use std::io::{self, BufRead};

use urlencoding::decode;

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output = decode(string.as_str()).expect("input was not valid UTF-8");
        println!("{}", output);
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
