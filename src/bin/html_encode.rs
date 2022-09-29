use std::io::{self, BufRead};

use html_escape;

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output = html_escape::encode_text(string.as_str());
        println!("{}", output);
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}

