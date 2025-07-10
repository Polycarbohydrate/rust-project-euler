// https://projecteuler.net/problem=70
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut n = 1;
    let mut minimum = 1e7;
    for number in 2..1e7 as u128 {
        if number % 100000 == 0 {
            println!("Number: {}", number);
        }
        let mut factors = prime_factors(number);
        factors.sort();
        factors.dedup();
        let mut product = number as f64;
        for factor in factors {
            product *= 1f64 - (1f64 / factor as f64);
        }
        let product = product as u128;
        let mut chars_of_number = number.to_string().chars().collect::<Vec<char>>();
        let mut chars_of_product = product.to_string().chars().collect::<Vec<char>>();
        chars_of_number.sort();
        chars_of_product.sort();
        if chars_of_number == chars_of_product {
            let ratio = number as f64 / product as f64;
            if ratio < minimum {
                minimum = ratio;
                n = number;
            }
        }
    }
    println!("================================");
    println!("Number: {}", n);
    println!("Minimum: {}", minimum);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}

fn prime_factors(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    let mut n = n;
    for i in 2..(n as f64).sqrt() as u128 + 1 {
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