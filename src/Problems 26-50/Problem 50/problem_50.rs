use std::time::Instant;

fn main() {
    let now = Instant::now();
    let limit = 1_000_000;
    let primes: Vec<u64> = (2..limit).filter(|&n| is_prime(n)).collect();
    let mut total_primes_used = 0;
    let mut big_prime = 0;
    for start in 0..primes.len() {
        let mut sum = 0;
        for end in start..primes.len() {
            sum += primes[end];
            if sum >= limit {
                break;
            }
            if is_prime(sum) && (end - start + 1) > total_primes_used {
                total_primes_used = end - start + 1;
                big_prime = sum;
            }
        }
    }

    println!("Total: {}", total_primes_used);
    println!("Biggest prime: {}", big_prime);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
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