// https://projecteuler.net/problem=43
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let mut storage: Vec<u64> = Vec::new();
    for number in 0_123_456_789u64..=9_876_543_210 {
        if number % 100_000_000 == 0 {
            println!("Checking number: {}", number);
        }
        if is_pandigital(number) {
            let num_chars: Vec<char> = number.to_string().chars().collect();
            let second_char = num_chars[1].to_digit(10).unwrap();
            let third_char = num_chars[2].to_digit(10).unwrap();
            let fourth_char = num_chars[3].to_digit(10).unwrap();
            let fifth_char = num_chars[4].to_digit(10).unwrap();
            let sixth_char = num_chars[5].to_digit(10).unwrap();
            let seventh_char = num_chars[6].to_digit(10).unwrap();
            let eighth_char = num_chars[7].to_digit(10).unwrap();
            let ninth_char = num_chars[8].to_digit(10).unwrap();
            let tenth_char = num_chars[9].to_digit(10).unwrap();
            let first_num = format!("{}{}{}", second_char, third_char, fourth_char);
            let second_num = format!("{}{}{}", third_char, fourth_char, fifth_char);
            let third_num = format!("{}{}{}", fourth_char, fifth_char, sixth_char);
            let fourth_num = format!("{}{}{}", fifth_char, sixth_char, seventh_char);
            let fifth_num = format!("{}{}{}", sixth_char, seventh_char, eighth_char);
            let sixth_num = format!("{}{}{}", seventh_char, eighth_char, ninth_char);
            let seventh_num = format!("{}{}{}", eighth_char, ninth_char, tenth_char);
            if first_num.parse::<u64>().unwrap() % 2 == 0
                && second_num.parse::<u64>().unwrap() % 3 == 0
                && third_num.parse::<u64>().unwrap() % 5 == 0
                && fourth_num.parse::<u64>().unwrap() % 7 == 0
                && fifth_num.parse::<u64>().unwrap() % 11 == 0
                && sixth_num.parse::<u64>().unwrap() % 13 == 0
                && seventh_num.parse::<u64>().unwrap() % 17 == 0
            {
                storage.push(number);
            }
        }
    }
    println!("{}", storage.iter().sum::<u64>());
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
    let answer = "answer.txt";
    let sum = storage.iter().sum::<u64>();
    let content = sum.to_string();
    fs::write(answer, content).unwrap();
}

fn is_pandigital(n: u64) -> bool {
    let mut chars: Vec<char> = n.to_string().chars().collect();
    chars.sort();
    let new_num = chars.iter().collect::<String>();
    if new_num == "0123456789" {
        return true
    }
    false
}