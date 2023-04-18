use ipnet::Ipv4Subnets;
use std::{
    io::{self, BufRead},
};

fn process<I: IntoIterator<Item = String>>(strings: I) {
    for string in strings {
        // Parse the input into two Ipv4Addr values
        let mut addrs = string.split('-');

        let start = addrs.next().unwrap();
        let end = addrs.next().unwrap();

        let subnets = Ipv4Subnets::new(
            start.parse().unwrap(),
            end.parse().unwrap(),
            1,
        );

        for subnet in subnets {
            println!("{}", subnet)
        }
    }
}

fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
