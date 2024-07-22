use toolkit::lib::foreach_input;
use urlencoding::decode;

fn process(string: String) {
    let output = decode(string.as_str()).expect("input was not valid UTF-8");
    println!("{}", output);
}

fn main() {
    foreach_input(process);
}
