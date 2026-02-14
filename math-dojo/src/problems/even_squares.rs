use crate::util::io::InputReader;

fn even_squares(v: &Vec<i64>) -> i128 {
    let mut sum: i128 = 0;
    for x in v {
        if x % 2 != 0 { continue; }
        let y = *x as i128;
        sum += y * y;
    }
    sum
}


pub fn run() {
    let mut reader = InputReader::new();
    let n: usize = reader.next();
    
    let mut v: Vec<i64> = Vec::new();
    for _ in 0..n {
        let x: i64 = reader.next();
        v.push(x);
    }

    // let sum = even_squares(&v);
    let sum = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| {
        let y = *x as i128;
        y * y
    })
    .sum::<i128>();
    println!("{}", sum)
}