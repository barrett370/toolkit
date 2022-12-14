use std::io::{self, BufRead};

use chrono::{DateTime, TimeZone, Utc};

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        let secs: i64;
        let nsecs: u32;
        match string.len() {
            10 => {
                secs = string.parse().expect("error parsing seconds");
                nsecs = 0;
            }
            13 => {
                secs = string[0..10].parse().expect("error parsing seconds");
                nsecs = string[11..].parse().expect("error parseing nanoseconds");
            }
            _ => {
                panic!("unsupported epoch length")
            }
        }
        let dt: DateTime<Utc> = Utc.timestamp_opt(secs, nsecs).unwrap();
        println!("{}", dt.format("%FT%H:%M:%SZ"));
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
