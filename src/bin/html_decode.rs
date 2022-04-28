use std::io;
use html_escape;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let decoded = html_escape::decode_html_entities(buffer.as_str());
    println!("{}", decoded);
}
