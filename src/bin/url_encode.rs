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
        Some(i) => {
            let v = vec![i];
            process(v)
        }
        None => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
    };
}
