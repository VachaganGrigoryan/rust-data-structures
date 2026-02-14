use crate::util::io::InputReader;


fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }

    if n == 2 { return true; }

    if n % 2 == 0 { return false; }

    let mut d = 3;

    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2
    }

    true
}



pub fn run() {
    let mut reader = InputReader::new();
    let n: i64 = reader.next();

    for x in 2..=n {
        if is_prime(x) {
            print!("{} ", x);
        }
    }
    println!();
}