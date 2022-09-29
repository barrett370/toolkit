use std::io::{self, BufRead};

use chrono::{DateTime, TimeZone, Utc};

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let epoch: i64 = string.parse().unwrap();
        let dt: DateTime<Utc> = Utc.timestamp(epoch, 0);
        println!("{}", dt.format("%FT%H:%M:%SZ"));
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
