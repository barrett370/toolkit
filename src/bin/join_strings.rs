

fn process<I: IntoIterator<Item=String>>(strings: I) {
    let res = strings.into_iter().collect::<Vec<String>>().join(", ");
    println!("{}", res)
}

fn main() {
    process(std::env::args().skip(1))
}