// https://projecteuler.net/problem=49
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut four_digit_primes = Vec::new();
    for num in 1000..10_000 {
        if is_prime(num) {
            four_digit_primes.push(num);
        }
    }
    for a in four_digit_primes.iter() {
        let chars: Vec<char> = a.to_string().chars().collect();
        let mut digits: Vec<usize> = chars.iter().map(|&c| c.to_digit(10).unwrap() as usize).collect();
        let permutations = permutations_fn(&mut digits, 4);
        let mut numbers_permutated = Vec::new();
        numbers_permutated.push(*a);
        for permutation in permutations {
            let number: u32 = permutation.iter().fold(0, |acc, &d| acc * 10 + d as u32);
            if number >= 1000 && number < 10_000 && is_prime(number) && number != *a {
                numbers_permutated.push(number);
            }
        }
        numbers_permutated.sort();
        numbers_permutated.dedup();
        let mut differences = Vec::new();
        if numbers_permutated.len() < 1 {
            continue;
        }
        for item in 0..numbers_permutated.len() - 1 {
            let difference: i64 = (numbers_permutated[item + 1] - numbers_permutated[item]) as i64;
            differences.push(difference);
        }
        differences.sort();
        for &diff in differences.iter() {
            for i in 0..numbers_permutated.len() {
                let first = numbers_permutated[i];
                let second = first + diff as u32;
                let third = second + diff as u32;
                if numbers_permutated.contains(&second) && numbers_permutated.contains(&third) {
                    let number = format!("{}{}{}", first, second, third);
                    println!("{}", number);
                    break
                }
            }
        }
    }
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}
fn permutations_fn(pending_permutation: &mut Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    if n == 1 {
        result.push(pending_permutation.clone());
    } else {
        for i in 0..n {
            result.extend(permutations_fn(pending_permutation, n - 1));
            if n % 2 == 0 {
                pending_permutation.swap(i, n - 1);
            } else {
                pending_permutation.swap(0, n - 1);
            }
        }
    }
    result
}
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}