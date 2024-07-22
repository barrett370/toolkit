pub mod lib {
    use std::io::{self, BufRead};

    fn process<I: IntoIterator<Item = String>>(f: &dyn Fn(String), strings: I) {
        for string in strings {
            f(string);
        }
    }

    // Intended to be called from main(). Takes input either from CLI args
    // or STDIN, and then calls the provided function for each input.
    pub fn foreach_input(f: fn(String)) {
        match std::env::args().len() {
            1 => process(&f, io::stdin().lock().lines().map(|ln| ln.unwrap())),
            _ => process(&f, std::env::args().skip(1)),
        };
    }
}
