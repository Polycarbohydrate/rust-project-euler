// https://projecteuler.net/problem=74
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut total_count = 0;
    for number in 1..=1_000_000 {
        if number % 10_000 == 0 {
            println!("Processing number: {}", number);
        }
        let mut count = 0;
        let mut seen_numbers = Vec::new();
        let mut current_number = number;
        while count != 60 {
            if seen_numbers.contains(&current_number) {
                break;
            }
            seen_numbers.push(current_number);
            let temp = factorial(current_number);
            current_number = temp;
            count += 1;
        }
        if count == 60 {
            total_count += 1;
        }
    }
    println!("Total count of numbers with 60 factorial chains: {}", total_count);
    println!("====================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}

fn factorial(n: u128) -> u128 {
    let chars = n.to_string().chars().collect::<Vec<char>>();
    let mut sum = 0;
    for c in chars {
        let num = c.to_digit(10).unwrap();
        if num == 0 || num == 1{
            sum += 1;
            continue
        }
        let mut smaller_nums = Vec::new();
        for l in 1..=num {
            smaller_nums.push(l as u128);
        }
        sum += smaller_nums.iter().product::<u128>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(145), 145);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(factorial(9), 362880);
        assert_eq!(factorial(10), 2);
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(11), 2);
    }
}