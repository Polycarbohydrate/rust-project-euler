// https://projecteuler.net/problem=95
use std::time::Instant;
use std::collections::HashSet;
fn main() {
    let now = Instant::now();
    let mut total_count = 0;
    let mut total_members = Vec::new();
    println!("Starting prime generation...");
    let temp = sieve_of_eratosthenes(100000);
    let primes: HashSet<u64> = temp.into_iter().collect();
    println!("Finished prime generation.");
    for n in 2..15_000 {
        if primes.contains(&n) {
            continue;
        }
        let factors = proper_divisors(n);
        let starting = factors.iter().sum::<u64>();
        if primes.contains(&starting) || starting >= 1_000_000  || starting == 0 {
            continue;
        }
        let mut temp = proper_divisors(starting).iter().sum();
        let mut count = 2;
        let mut seen = HashSet::new();
        let mut members = vec![n, starting];
        while temp != n && temp < 1_000_000 && !primes.contains(&temp) {
            if seen.contains(&temp) {
                break;
            }
            seen.insert(temp);
            temp = proper_divisors(temp).iter().sum();
            count += 1;
            members.push(temp);
        }
        if temp == n {
            if count > total_count {
                total_count = count;
                total_members = members;
            }
        }
    }
    total_members.sort();
    println!("Longest chain has {} elements:", total_count);
    println!("{:?}", total_members);
    println!("================================");
    println!("{} s", now.elapsed().as_secs());
}

fn proper_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in 1..=n / 2 {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn sieve_of_eratosthenes(n: usize) -> Vec<u64> {
    let mut integers = vec![true; n + 1];
    integers[0] = false;
    integers[1] = false;
    for i in 2..(n as f64).sqrt() as usize {
        if integers[i] {
            let mut j = i * i;
            while j < n {
                integers[j] = false;
                j += i;
            }
        }
    }
    let mut primes = Vec::new();
    for (num, status) in integers.into_iter().enumerate() {
        if status {
            primes.push(num as u64);
        }
    }
    primes
}