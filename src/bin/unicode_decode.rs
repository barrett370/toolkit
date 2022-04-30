use std::io::{self, BufRead};

use unescape::unescape;

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output =
            unescape(string.as_str()).expect("input could not be decoded as valid unicode");
        println!("{}", output);
    }
}

fn main() {
    match std::env::args().nth(1) {
        Some(_) => process(std::env::args()),
        None => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
    };
}
