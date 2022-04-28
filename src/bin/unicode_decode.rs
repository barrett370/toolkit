use std::io;
use unescape::unescape;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let decoded = unescape(buffer.as_str()).unwrap();
    println!("{}", decoded);
}
