// https://projecteuler.net/problem=47
use std::time::Instant;
fn main() {
    let now = Instant::now();
    for number in 650..1_000_000 {
        if number % 100_000 == 0 {
            println!("{} numbers checked", number);
        }
        let first_number = number;
        let second_number = number + 1;
        let third_number = number + 2;
        let fourth_number = number + 3;
        if is_prime(first_number) || is_prime(second_number) || is_prime(third_number) || is_prime(fourth_number) {
            continue;
        }
        let mut first_factors = factorize(first_number);
        let mut second_factors = factorize(second_number);
        let mut third_factors = factorize(third_number);
        let mut fourth_factors = factorize(fourth_number);
        first_factors.sort();
        second_factors.sort();
        third_factors.sort();
        fourth_factors.sort();
        first_factors.dedup();
        second_factors.dedup();
        third_factors.dedup();
        fourth_factors.dedup();
        if first_factors.len() == 4 && second_factors.len() == 4 && third_factors.len() == 4 && fourth_factors.len() == 4 {
            println!("The four consecutive integers are: {}, {}, {}, {}", first_number, second_number, third_number, fourth_number);
            println!("Their prime factors are: {:?} {:?} {:?} {:?}", first_factors, second_factors, third_factors, fourth_factors);
            break;
        }
    }
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}

fn factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
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