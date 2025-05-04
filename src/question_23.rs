// https://projecteuler.net/problem=23
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut non_abundant_nums: Vec<i64> = Vec::new();
    let mut sum_abundant_nums: Vec<i64> = Vec::new();
    let mut abundant_nums: Vec<i64> = Vec::new();
    for num in 1..=28123 {
        let sum = sums(num);
        if sum > num {
            abundant_nums.push(num);
        }
    }
    for abundant_num in abundant_nums.clone() {
        for abundant_num2 in abundant_nums.clone() {
            let sum = abundant_num + abundant_num2;
            if sum <= 28123 {
                if !sum_abundant_nums.contains(&sum) {
                    sum_abundant_nums.push(sum);
                }
            }
        }
    }
    for num in 1..=28123 {
        if !sum_abundant_nums.contains(&num) {
            non_abundant_nums.push(num);
        }
    }

    println!("Runtime: {} seconds", start.elapsed().as_secs());
    println!("{}", non_abundant_nums.iter().sum::<i64>());
}

fn sums(num: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=(num / 2) {
        if num % i == 0 {
            sum += i;
        }
    }
    sum
}
