// https://projecteuler.net/problem=89
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let mut original_char_count = 0;
    let contents = fs::read_to_string("0089_roman.txt").expect("Something went wrong reading the file");
    for line in contents.lines() {
        original_char_count += line.len();
    }
    println!("Original char count: {}", original_char_count);
    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let mut value = 0;
        let chars: Vec<char> = line.chars().collect();
        let final_num_index = chars.len() - 1;
        for (index, char) in chars.iter().enumerate() {
            let current_value = get_value(*char);
            if index < final_num_index {
                let next_value = get_value(chars[index + 1]);
                if current_value < next_value {
                    value -= current_value;
                } else {
                    value += current_value;
                }
            } else {
                value += current_value;
            }
        }
        numbers.push(value);
    }
    let mut minimal_char_count = 0;
    for num in numbers {
        let mut roman = String::new();
        let mut num = num;
        while num != 0 {
            if num >= 1000 {
                roman.push_str("M");
                num -= 1000;
            } else if num >= 900 {
                roman.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                roman.push_str("D");
                num -= 500;
            } else if num >= 400 {
                roman.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                roman.push_str("C");
                num -= 100;
            } else if num >= 90 {
                roman.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                roman.push_str("L");
                num -= 50;
            } else if num >= 40 {
                roman.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                roman.push_str("X");
                num -= 10;
            } else if num >= 9 {
                roman.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                roman.push_str("V");
                num -= 5;
            } else if num >= 4 {
                roman.push_str("IV");
                num -= 4;
            } else if num >= 1 {
                roman.push_str("I");
                num -= 1;
            }
        }
        minimal_char_count += roman.len();
    }
    println!("Minimal char count: {}", minimal_char_count);
    let difference = original_char_count - minimal_char_count;
    println!("Difference: {}", difference);
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}

fn get_value(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
