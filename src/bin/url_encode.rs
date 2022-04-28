use std::io;
use urlencoding::encode;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let encoded = encode(buffer.as_str());
    println!("{}", encoded);
}
