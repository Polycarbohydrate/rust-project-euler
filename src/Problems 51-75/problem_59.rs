// https://projecteuler.net/problem=59
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("0059_cipher.txt").expect("Unable to read file");
    let vec_numbers: Vec<u8> = input.split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    println!("Total encrypted ASCII numbers: {}", vec_numbers.len());
    let mut possible_keys = Vec::new();
    for first_letter in 97..=122 {
        for second_letter in 97..=122 {
            for third_letter in 97..=122 {
                let mut line_up = Vec::new();
                for _ in 0..(vec_numbers.len() / 3) {
                    line_up.push(first_letter);
                    line_up.push(second_letter);
                    line_up.push(third_letter);
                }
                for index in 0..vec_numbers.len() {
                    line_up[index] = line_up[index] ^ vec_numbers[index];
                }
                let mut count = 0;
                for letter in &line_up {
                    if letter == &101 || letter == &69 {
                        count += 1;
                    }
                }
                if count == line_up.len() {
                    possible_keys.push((first_letter, second_letter, third_letter));
                }

                if (count as f64 / line_up.len() as f64) * 100.0 > 10.0 && (count as f64 / line_up.len() as f64) * 100.0 < 14.0 {
                    possible_keys.push((first_letter, second_letter, third_letter));
                }
            }
        }
    }
    println!("Possible Keys: {:?}", possible_keys);
    println!("================================");
    println!("First key output: ");
    let mut line_up = Vec::new();
    for _ in 0..(vec_numbers.len() / 3) {
        line_up.push(possible_keys[0].0);
        line_up.push(possible_keys[0].1);
        line_up.push(possible_keys[0].2);
    }
    let mut letters = Vec::new();
    for index in 0..vec_numbers.len() {
        letters.push((line_up[index] ^ vec_numbers[index]) as char);
    }
    println!("{}", letters.iter().collect::<String>());
    println!("================================");
    println!("Second key output: ");
    line_up.clear();
    for _ in 0..(vec_numbers.len() / 3) {
        line_up.push(possible_keys[1].0);
        line_up.push(possible_keys[1].1);
        line_up.push(possible_keys[1].2);
    }
    letters.clear();
    for index in 0..vec_numbers.len() {
        letters.push((line_up[index] ^ vec_numbers[index]) as char);
    }
    println!("{}", letters.iter().collect::<String>());
    println!("================================");
    println!("Third key output: ");
    line_up.clear();
    for _ in 0..(vec_numbers.len() / 3) {
        line_up.push(possible_keys[2].0);
        line_up.push(possible_keys[2].1);
        line_up.push(possible_keys[2].2);
    }
    letters.clear();
    for index in 0..vec_numbers.len() {
        letters.push((line_up[index] ^ vec_numbers[index]) as char);
    }
    println!("{}", letters.iter().collect::<String>());
    println!("================================");
    println!("Fourth key output: ");
    line_up.clear();
    for _ in 0..(vec_numbers.len() / 3) {
        line_up.push(possible_keys[3].0);
        line_up.push(possible_keys[3].1);
        line_up.push(possible_keys[3].2);
    }
    letters.clear();
    let mut final_vec = Vec::new();
    for index in 0..vec_numbers.len() {
        letters.push((line_up[index] ^ vec_numbers[index]) as char);
        final_vec.push((line_up[index] ^ vec_numbers[index]) as u32);
    }
    println!("{}", letters.iter().collect::<String>());
    println!("================================");
    println!("The key is (101, 120, 112) or the 3 lowercase letters 'e', 'x', 'p'");
    println!("The sum of the decrypted ASCII values are: {}", final_vec.iter().sum::<u32>());
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}