// https://projecteuler.net/problem=42
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut total_sum = 0;
    let contents = std::fs::read_to_string("0042_words.txt").expect("Failed to read file");
    let words: Vec<&str> = contents.split(',').collect();
    for word in words {
        let mut sum = 0;
        let formatted_word = word.trim_matches('"');
        let chars = formatted_word.chars().collect::<Vec<char>>();
        for character in chars {
            match character {
                'A' => sum += 1,
                'B' => sum += 2,
                'C' => sum += 3,
                'D' => sum += 4,
                'E' => sum += 5,
                'F' => sum += 6,
                'G' => sum += 7,
                'H' => sum += 8,
                'I' => sum += 9,
                'J' => sum += 10,
                'K' => sum += 11,
                'L' => sum += 12,
                'M' => sum += 13,
                'N' => sum += 14,
                'O' => sum += 15,
                'P' => sum += 16,
                'Q' => sum += 17,
                'R' => sum += 18,
                'S' => sum += 19,
                'T' => sum += 20,
                'U' => sum += 21,
                'V' => sum += 22,
                'W' => sum += 23,
                'X' => sum += 24,
                'Y' => sum += 25,
                'Z' => sum += 26,
                _ => println!("Unrecognized character: {}", character),
            }
        }
        if is_triangle_number(sum) {
            total_sum += 1;
        }
    }
    println!("Total sum is {}", total_sum);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}

fn is_triangle_number(n: u32) -> bool {
    let mut counter = 1;
    let mut triangle_number = 0;
    while triangle_number < n {
        let result = ((0.5 * counter as f64) * (counter as f64 + 1.0)) as u32;
        triangle_number = result;
        counter += 1;
        if triangle_number == n {
            return true;
        }
    }
    false
}