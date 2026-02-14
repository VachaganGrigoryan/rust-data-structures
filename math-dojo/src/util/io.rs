use std::io::{self, Read};

pub struct InputReader {
    it: std::str::SplitWhitespace<'static>,
}

impl InputReader {
    pub fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read input");
        // Leak the string to get a 'static lifetime
        let input: &'static str = Box::leak(input.into_boxed_str());
        Self {
            it: input.split_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().expect("No more input").parse::<T>().ok().expect("Failed to parse input")
    }
}