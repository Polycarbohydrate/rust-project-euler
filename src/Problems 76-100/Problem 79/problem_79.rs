// https://projecteuler.net/problem=79
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let passwords = fs::read_to_string("0079_keylog.txt").unwrap();
    let mut total_code = String::new();
    for line in passwords.lines() {
        let digits: Vec<char> = line.chars().collect();
        let first = digits[0];
        let second = digits[1];
        let third = digits[2];
        let code = format!("{first} -> {second} -> {third}\n");
        total_code += &code;
    }
    println!("{}", total_code.trim_end());
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}