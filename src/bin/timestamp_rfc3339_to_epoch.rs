use chrono::{DateTime, FixedOffset};
use toolkit::lib::foreach_input;

fn process(string: String) {
    let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&string).unwrap();
    println!("{}", dt.format("%s"));
}

fn main() {
    foreach_input(process);
}
