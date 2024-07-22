use chrono::{DateTime, TimeZone, Utc};
use toolkit::lib::foreach_input;

fn process(string: String) {
    let epoch: i64 = string.parse().unwrap();
    let dt: DateTime<Utc> = Utc.timestamp_opt(epoch, 0).unwrap();
    println!("{}", dt.format("%FT%H:%M:%SZ"));
}

fn main() {
    foreach_input(process);
}
