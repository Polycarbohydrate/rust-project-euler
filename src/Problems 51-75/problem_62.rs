// https://projecteuler.net/problem=62
use std::time::Instant;
use std::collections::HashMap;
fn main() {
    let now = Instant::now();
    let mut unique_digits: HashMap<Vec<char>, i8> = HashMap::new();
    let mut found_chars = Vec::new();
    for i in 300i64.. {
        let num = i * i * i;
        let mut chars = num.to_string().chars().collect::<Vec<char>>();
        chars.sort_unstable();
        *unique_digits.entry(chars.clone()).or_insert(0) += 1;
        if unique_digits.get(&chars).unwrap() == &5 {
            println!("The largest cube for which exactly five permutations of its digits are cube is: {}", num);
            for char in chars {
                found_chars.push(char);
            }
            break;
        }
    }
    found_chars.sort_unstable();
    let mut count = 0;
    println!("Cubes with exactly five permutations:");
    for i in 300i64.. {
        let num = i * i * i;
        let mut chars = num.to_string().chars().collect::<Vec<char>>();
        chars.sort_unstable();
        if chars == found_chars {
            count += 1;
            println!("Cube permutation: {}", num);
        }
        if count == 5 {
            break;
        }
    }
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}