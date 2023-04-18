use toolkit::lib::foreach_input;
use urlencoding::encode;

fn process(string: String) {
    let output = encode(string.as_str());
    println!("{}", output);
}

fn main() {
    foreach_input(process);
}
