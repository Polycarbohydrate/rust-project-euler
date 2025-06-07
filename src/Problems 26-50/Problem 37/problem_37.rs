// https://projecteuler.net/problem=37
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut sum = 0;
    for initial_number in 11..1_000_000 {
        if initial_number % 2 == 0 {
            continue;
        }
        if !is_prime(initial_number) {
            continue;
        }
        let mut left = true;
        let mut right = true;
        let mut individual_digits: Vec<char> = initial_number.to_string().chars().collect();
        let mut individual_digits_2: Vec<char> = individual_digits.clone();
        let length = individual_digits.len();
        for _ in 0..length - 1 {
            let length = individual_digits.len();
            individual_digits.remove(length - 1);
            let rest_of_number: u64 = individual_digits.iter().collect::<String>().parse().unwrap_or(0);
            if !is_prime(rest_of_number) {
                left = false;
                break;
            }
        }
        for _ in 0..length - 1 {
            individual_digits_2.remove(0);
            let rest_of_number_2: u64 = individual_digits_2.iter().collect::<String>().parse().unwrap_or(0);
            if !is_prime(rest_of_number_2) {
                right = false;
                break;
            }
        }
        if !left || !right {
            continue;
        }
        sum += initial_number;
    }
    println!("{}", sum);
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
