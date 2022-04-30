use html_escape;
use std::io::{self, BufRead};

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let output = html_escape::encode_text(string.as_str());
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
