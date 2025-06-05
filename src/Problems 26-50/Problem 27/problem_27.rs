// https://projecteuler.net/problem=27
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut storage: HashMap<(i64,i64), i64> = HashMap::new();
    for a in -999..1000 {
        for b in -999..1000 {
            // Formula: n^2 + an + b
            let mut n: i64 = 0;
            loop {
                let num = (n.pow(2)) + (a * n) + b;
                let is_prime = test_is_prime(num);
                if is_prime == true {
                    n += 1;
                } else {
                    break;
                }
            }
            if n != 0 {
                storage.insert((a, b), n);
            }
        }
    }
    if let Some(((a, b), n)) = storage.iter().max_by_key(|entry| entry.1) {
        println!("Max: (a, b) = ({}, {}), n = {}", a, b, n);
        println!("Product: {}", a * b);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
fn test_is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}