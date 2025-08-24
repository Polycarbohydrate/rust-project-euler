// https://projecteuler.net/problem=55
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut count = 0;
    for n in 1..10_000 {
        let mut counter = 0;
        let mut sum = n;
        let mut chars = sum.to_string().chars().collect::<Vec<char>>();
        let mut rev_chars = chars.clone();
        rev_chars.reverse();
        if rev_chars == chars {
            let original_n = chars.iter().collect::<String>().parse::<i128>().unwrap();
            let rev_n = rev_chars.iter().collect::<String>().parse::<i128>().unwrap();
            sum = original_n + rev_n;
            chars = sum.to_string().chars().collect::<Vec<char>>();
            rev_chars = chars.clone();
            rev_chars.reverse();
            counter += 1;
        }
        while rev_chars != chars {
            let original_n = chars.iter().collect::<String>().parse::<i128>().unwrap();
            let rev_n = rev_chars.iter().collect::<String>().parse::<i128>().unwrap();
            sum = original_n + rev_n;
            chars = sum.to_string().chars().collect::<Vec<char>>();
            rev_chars = chars.clone();
            rev_chars.reverse();
            counter += 1;
            if counter > 49 {
                count += 1;
                break;
            }
        }
    }
    println!("Answer: {}", count);
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}