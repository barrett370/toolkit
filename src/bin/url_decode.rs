use std::io;
use urlencoding::decode;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let decoded = decode(buffer.as_str()).expect("UTF-8");
    println!("{}", decoded);
}
