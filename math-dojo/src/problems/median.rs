use crate::util::io::InputReader;

pub fn run() {
    let mut reader = InputReader::new();
    let n: usize = reader.next();

    let mut arr: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(reader.next::<i64>());
    }

    arr.sort();

    if n % 2 == 1 { 
        let m = arr[n / 2];
        println!("{}", m);
    } else {
        let x = arr[n / 2] as f64;
        let y = arr[n / 2 - 1] as f64;
        let m = (x + y) / 2.0;
        println!("{}", m);
    }
}