// https://projecteuler.net/problem=41
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut pandigital_primes: Vec<u64> = Vec::new();
    for integer in 11..=8_000_000 {
        if integer % 2 == 0 {
            continue;
        }
        if is_prime(integer) {
            let mut integer_chars: Vec<char> = integer.to_string().chars().collect();
            let integer_len = integer_chars.len();
            if integer_len == 2 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "12" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 3 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "123" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 4 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "1234" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 5 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "12345" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 6 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "123456" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 7 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "1234567" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 8 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "12345678" {
                    pandigital_primes.push(integer);
                }
            } else if integer_len == 9 {
                integer_chars.sort();
                let new_integer: String = integer_chars.iter().collect();
                if new_integer == "123456789" {
                    pandigital_primes.push(integer);
                }
            } else {
                println!("Something went wrong");
            }
        }
    }
    pandigital_primes.sort();
    let len = pandigital_primes.len();
    println!("{}", pandigital_primes[len -1]);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}

fn is_prime(n: u64) -> bool {
    for divisors in 2..=((n as f64).sqrt() as u64) {
        if n % divisors == 0 {
            return false;
        }
    }
    true
}