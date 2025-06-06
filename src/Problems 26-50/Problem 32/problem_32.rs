// https://projecteuler.net/problem=32
use std::time::Instant;
use std::collections::HashSet;
fn main() {
    let now = Instant::now();
    let mut sum = Vec::new();
    for multiplicand in 1..9999 {
        for multiplier in 1..999 {
            let mut chars = Vec::new();
            let product = multiplicand * multiplier;
            if product > 9999 {
                continue;
            }
            let a_product = product.to_string().chars().collect::<Vec<char>>();
            let b_multiplicand = multiplicand.to_string().chars().collect::<Vec<char>>();
            let c_multiplier = multiplier.to_string().chars().collect::<Vec<char>>();
            chars.extend(b_multiplicand);
            chars.extend(c_multiplier);
            chars.extend(a_product);
            let len = chars.len();
            if len != 9 {
                continue;
            }
            if check_for_duplicates(&chars) {
                sum.push(product);
            }
        }
    }
    sum.sort();
    sum.dedup();
    println!("{}", sum.iter().sum::<u64>());
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}

fn check_for_duplicates(chars: &Vec<char>) -> bool {
    let mut seen = HashSet::new();
    for item in chars {
        if item == &'0' || !seen.insert(item) {
            return false;
        }
    }
    true
}