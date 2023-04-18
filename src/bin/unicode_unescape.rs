use toolkit::lib::foreach_input;
use unescape::unescape;

fn process(string: String) {
    let output = unescape(string.as_str()).expect("input could not be decoded as valid unicode");
    println!("{}", output);
}

fn main() {
    foreach_input(process);
}
