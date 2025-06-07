// https://projecteuler.net/problem=36
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut sum = 0;
    for initial_number in 1..1_000_000 {
        let chars: Vec<char> = initial_number.to_string().chars().collect();
        let reversed: String = chars.iter().rev().collect();
        let original: String = initial_number.to_string();
        if original == reversed {
            let base_2 = format!("{:b}", initial_number);
            let chars_2: Vec<char> = base_2.chars().collect();
            let reversed_2: String = chars_2.iter().rev().collect();
            if base_2  == reversed_2  {
                sum += initial_number;
            }
        }
    }
    println!("{}", sum);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}
