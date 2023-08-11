use std::io::{self, BufRead};

fn process<I: IntoIterator<Item=String>>(strings: I) {
    println!(
        "{}",
        strings.into_iter().collect::<Vec<String>>().join(", ")
    )
}
fn main() {
    match std::env::args().len() {
        1 => process(io::stdin().lock().lines().map(|ln| ln.unwrap())),
        _ => process(std::env::args().skip(1)),
    };
}
