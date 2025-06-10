// https://projecteuler.net/problem=46
use std::time::Instant;
fn main() {
    let now = Instant::now();
    for number in 33..10000 {
        if number % 2 == 0 || is_prime(number) {
            continue;
        }
        let mut primes = Vec::new();
        for num in 2..number {
            if is_prime(num) {
                primes.push(num);
            }
        }
        // n=p+2×k^2
        for prime in &primes {
            let k = ((number - prime) as f64 / 2.0).sqrt() as u64;
            if k * k * 2 + prime == number {
                break;
            }
            if prime == primes.last().unwrap() {
                println!("{}", number);
                println!("================================");
                println!("{} milliseconds of runtime", now.elapsed().as_millis());
                return;
            }
        }
    }
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
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