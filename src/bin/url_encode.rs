use std::io::{self, BufRead};

use urlencoding::encode;

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output = encode(string.as_str());
        println!("{}", output);
    }
}

fn main() {
    match std::env::args().nth(1) {
        Some(_) => process(std::env::args()),
        None => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
    };
}
