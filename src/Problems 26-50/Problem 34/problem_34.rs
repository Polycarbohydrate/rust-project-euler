// https://projecteuler.net/problem=34
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut sum = 0;
    for curious_number in 3..=9_999_999 {
        let individual_digits = curious_number.to_string().chars().collect::<Vec<char>>();
        let mut sum_of_factorials = 0;
        for digit in individual_digits {
            let digit = digit.to_digit(10).unwrap();
            let product = (1..=digit).product::<u32>();
            sum_of_factorials += product;
        }
        if sum_of_factorials == curious_number {
            println!("Found curious number: {}", curious_number);
            sum += curious_number;
        }
    }
    println!("{sum}");
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}
