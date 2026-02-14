use crate::util::io::InputReader;

fn sum_squares(n: usize) -> i128 {
    let mut sum = 0;
    for i in 1..=n {
        let x = i as i128;
        sum += x * x;
    }
    sum
}


pub fn run() {
    let mut reader = InputReader::new();
    let n: usize = reader.next();

    let sum = sum_squares(n);
    println!("{}", sum)
}