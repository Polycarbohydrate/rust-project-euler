// https://projecteuler.net/problem=35
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut circular_primes = Vec::new();
    for initial_nums in 2..1_000_000 {
        if !check_is_prime(initial_nums) {
            continue;
        }
        let len = initial_nums.to_string().len();
        if len == 1 {
            circular_primes.push(initial_nums);
            continue;
        } else if len == 2 {
            let mut chars: Vec<char> = initial_nums.to_string().chars().collect();
            chars.reverse();
            let reversed_num: u128 = chars.iter().collect::<String>().parse().unwrap();
            if check_is_prime(reversed_num) {
                circular_primes.push(initial_nums);
            }
        } else if len == 3 {
            let last_char = initial_nums % 10;
            let rest_chars = initial_nums / 10;
            let shifted_num_1 = last_char * 100 + rest_chars;
            let last_char_2 = shifted_num_1 % 10;
            let rest_chars_2 = shifted_num_1 / 10;
            let shifted_num_2 = last_char_2 * 100 + rest_chars_2;
            if check_is_prime(shifted_num_1) && check_is_prime(shifted_num_2) {
                circular_primes.push(initial_nums);
            }
        } else if len == 4 {
            let last_char = initial_nums % 10;
            let rest_chars = initial_nums / 10;
            let shifted_num_1 = last_char * 1000 + rest_chars;
            let last_char_2 = shifted_num_1 % 10;
            let rest_chars_2 = shifted_num_1 / 10;
            let shifted_num_2 = last_char_2 * 1000 + rest_chars_2;
            let last_char_3 = shifted_num_2 % 10;
            let rest_chars_3 = shifted_num_2 / 10;
            let shifted_num_3 = last_char_3 * 1000 + rest_chars_3;
            if check_is_prime(shifted_num_1) && check_is_prime(shifted_num_2) && check_is_prime(shifted_num_3) {
                circular_primes.push(initial_nums);
            }
        } else if len == 5 {
            let last_char = initial_nums % 10;
            let rest_chars = initial_nums / 10;
            let shifted_num_1 = last_char * 10000 + rest_chars;
            let last_char_2 = shifted_num_1 % 10;
            let rest_chars_2 = shifted_num_1 / 10;
            let shifted_num_2 = last_char_2 * 10000 + rest_chars_2;
            let last_char_3 = shifted_num_2 % 10;
            let rest_chars_3 = shifted_num_2 / 10;
            let shifted_num_3 = last_char_3 * 10000 + rest_chars_3;
            let last_char_4 = shifted_num_3 % 10;
            let rest_chars_4 = shifted_num_3 / 10;
            let shifted_num_4 = last_char_4 * 10000 + rest_chars_4;
            if check_is_prime(shifted_num_1) && check_is_prime(shifted_num_2) && check_is_prime(shifted_num_3) && check_is_prime(shifted_num_4) {
                circular_primes.push(initial_nums);
            }
        } else if len == 6 {
            let last_char = initial_nums % 10;
            let rest_chars = initial_nums / 10;
            let shifted_num_1 = last_char * 100000 + rest_chars;
            let last_char_2 = shifted_num_1 % 10;
            let rest_chars_2 = shifted_num_1 / 10;
            let shifted_num_2 = last_char_2 * 100000 + rest_chars_2;
            let last_char_3 = shifted_num_2 % 10;
            let rest_chars_3 = shifted_num_2 / 10;
            let shifted_num_3 = last_char_3 * 100000 + rest_chars_3;
            let last_char_4 = shifted_num_3 % 10;
            let rest_chars_4 = shifted_num_3 / 10;
            let shifted_num_4 = last_char_4 * 100000 + rest_chars_4;
            let last_char_5 = shifted_num_4 % 10;
            let rest_chars_5 = shifted_num_4 / 10;
            let shifted_num_5 = last_char_5 * 100000 + rest_chars_5;
            if check_is_prime(shifted_num_1) && check_is_prime(shifted_num_2) && check_is_prime(shifted_num_3) && check_is_prime(shifted_num_4) && check_is_prime(shifted_num_5) {
                circular_primes.push(initial_nums);
            }
        }
    }
    println!("{:?}", circular_primes);
    println!("There are {} circular primes below 1,000,000", circular_primes.len());
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}
fn check_is_prime(num: u128) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u128) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
