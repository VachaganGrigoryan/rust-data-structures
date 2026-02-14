use crate::util::io::InputReader;


fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}



pub fn run() {
    let mut reader = InputReader::new();
    let a: i64 = reader.next();
    let b: i64 = reader.next();
    println!("{}", gcd(a, b));
}