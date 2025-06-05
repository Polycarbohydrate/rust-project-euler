// https://projecteuler.net/problem=30
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut sum: u64 = 0;
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            let total_power = (a as u64).pow(5) + (b as u64).pow(5) + (c as u64).pow(5) + (d as u64).pow(5) + (e as u64).pow(5) + (f as u64).pow(5);
                            let number = format!("{}{}{}{}{}{}", a, b, c, d, e, f);
                            let actual_number = number.parse::<u64>().unwrap();
                            if total_power == actual_number && actual_number != 1 {
                                sum += actual_number;
                                println!("{}", actual_number);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Sum: {}", sum);
    println!("{}", now.elapsed().as_millis());
}