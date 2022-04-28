use std::io;
use html_escape;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let encoded = html_escape::encode_text(buffer.as_str());
    println!("{}", encoded);
}
