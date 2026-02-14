use crate::util::io::InputReader;


pub fn run() {
    let mut reader = InputReader::new();
    
    let n: usize = reader.next();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(reader.next::<i64>());
    }

    let mut b: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        b.push(reader.next::<i64>());
    }

    // let mut sum: i128 = 0;
    // for i in 0..n {
    //     sum += a[i] as i128 * b[i] as i128; 
    // }

    let sum: i128 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            // let xi = *x as i128;
            // let yi = *y as i128;

            // xi * yi
            (*x as i128) * (*y as i128)
        })
        .sum::<i128>();


    println!("{}", sum);
}