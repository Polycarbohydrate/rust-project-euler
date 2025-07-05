// https://projecteuler.net/problem=87
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut prime_squares = Vec::new();
    let mut prime_cubes = Vec::new();
    let mut prime_quartics = Vec::new();
    let mut answers = Vec::new();
    for num in 2..=50_000_000f64.sqrt() as u64 {
        if is_prime(num) {
            let result = num.pow(2);
            if result < 50_000_000 {
                prime_squares.push(result);
            }
            let result = num.pow(3);
            if result < 50_000_000 {
                prime_cubes.push(result);
            }
            let result = num.pow(4);
            if result < 50_000_000 {
                prime_quartics.push(result);
            }
        }
    }
    println!("Prime sorting done in: {}", now.elapsed().as_millis());
    println!("Squares: {}", prime_squares.len());
    println!("Cubes: {}", prime_cubes.len());
    println!("Quartics: {}", prime_quartics.len());
    let mut counter = 0;
    for square in &prime_squares {
        if counter % 10 == 0 {
            println!("Percent done: {}", (counter as f64 / 908f64) * 100.0);
        }
        counter += 1;
        for cube in &prime_cubes {
            for quartic in &prime_quartics {
                let result = square + cube + quartic;
                if result < 50_000_000 {
                    if !answers.contains(&result) {
                        answers.push(result);
                    }
                }
            }
        }
    }
    println!("Answer: {}", answers.len());
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}