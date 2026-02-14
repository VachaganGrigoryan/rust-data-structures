use crate::util::io::InputReader;


pub fn run() {
    let mut reader = InputReader::new();
    
    let n: usize = reader.next();
    let q: usize = reader.next();


    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(reader.next::<i64>());
    }

    let mut pref: Vec<i128> = vec![0; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + a[i] as i128;
    }

    for _ in 0..q {
        let l: usize = reader.next();
        let r: usize = reader.next();
        let sum = pref[r] - pref[l-1];
        println!("{}", sum);
    }
}