use html_escape;
use toolkit::lib::foreach_input;

fn process(string: String) {
    let output = html_escape::decode_html_entities(string.as_str());
    println!("{}", output);
}

fn main() {
    foreach_input(process)
}
