// https://projecteuler.net/problem=72
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut total_count = 0;
    for d in 2..=1_000_000 {
        if d % 100_000 == 0 {
            println!("At d = {}", d);
        }
        let mut factors = prime_factors(d);
        factors.sort();
        factors.dedup();
        let mut product = d as f64;
        for factor in factors {
            let temp = 1.0 - (1.0 / factor as f64);
            product *= temp;
        }
        total_count += product as u64;
    }
    println!("Total count: {}", total_count);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut n = n;
    for i in 2..=(n as f64).sqrt() as u64 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}