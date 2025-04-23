use clap::{Arg, Command};
use random_string::{charsets, generate};

fn main() {
    let matches = Command::new("rand_string")
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .default_value("16"),
        )
        .arg(
            Arg::new("charset")
                .short('c')
                .long("charset")
                .default_value(charsets::ALPHANUMERIC),
        )
        .get_matches();

    let length_string: &String = matches.get_one("length").expect("a default value");
    let charset: &String = matches.get_one("charset").expect("a default value");

    let length: usize = length_string.parse().expect("a valid usize");

    println!("{}", generate(length.to_owned(), charset.to_owned()))
}
