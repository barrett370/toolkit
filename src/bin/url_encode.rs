use std::io::{self, BufRead};

use urlencoding::encode;

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output = encode(string.as_str());
        println!("{}", output);
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
