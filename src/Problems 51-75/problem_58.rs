// https://projecteuler.net/problem=58
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut total_diagonal_count = 13;
    let mut prime_diagonal_count = 8;
    let mut side_length = 7;
    let mut quotient = (prime_diagonal_count as f64 / total_diagonal_count as f64) * 100.0;
while quotient > 10.0 {
        let new_quotient = (prime_diagonal_count as f64 / total_diagonal_count as f64) * 100.0;
        quotient = new_quotient;
        side_length += 2;
        let last_number = side_length * side_length;
        let first_number = (side_length - 2) * (side_length - 2) + 1;
        let mut new_ring_numbers = Vec::new();
        for num_new in first_number..=last_number {
            new_ring_numbers.push(num_new);
        }
        let cutoff_point = new_ring_numbers.len() / 4;
        let section_1 = &new_ring_numbers[0..cutoff_point];
        let section_2 = &new_ring_numbers[cutoff_point..cutoff_point * 2];
        let section_3 = &new_ring_numbers[cutoff_point * 2..cutoff_point * 3];
        let section_4 = &new_ring_numbers[cutoff_point * 3..];
        let diagonal_num1 = section_1[section_1.len() - 1];
        let diagonal_num2 = section_2[section_2.len() - 1];
        let diagonal_num3 = section_3[section_3.len() - 1];
        let diagonal_num4 = section_4[section_4.len() - 1];
        if is_prime(diagonal_num1) {
            prime_diagonal_count += 1;
        }
        if is_prime(diagonal_num2) {
            prime_diagonal_count += 1;
        }
        if is_prime(diagonal_num3) {
            prime_diagonal_count += 1;
        }
        if is_prime(diagonal_num4) {
            prime_diagonal_count += 1;
        }
        total_diagonal_count += 4;
    }
    println!("================================");
    println!("Total diagonal count: {}", total_diagonal_count);
    println!("Prime diagonal count: {}", prime_diagonal_count);
    println!("Ratio: {}", quotient);
    println!("Side length: {}", side_length - 2);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
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