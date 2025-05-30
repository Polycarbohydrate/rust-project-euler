// https://projecteuler.net/problem=22
use std::fs::File;
use std::io::Read;

fn main() {
    let mut total_sum = 0;
    let mut file = File::open("0022_names.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut names: Vec<String> = Vec::new();
    let formatted_contents = contents.replace("\"", "");
    for name in formatted_contents.split(',') {
        names.push(name.to_string());
    }
    names.sort();
    for name in names.clone() {
        let num1 = letter_to_num(&name);
        let num2 = names.iter().position(|x| x == &name).unwrap() as u128 + 1;
        let final_num = num1 * num2;
        total_sum += final_num;
    }
    println!("Total sum: {}", total_sum);
}

fn letter_to_num(name: &str) -> u128 {
    let mut letters: Vec<String> = Vec::new();
    for letter in name.chars() {
        letters.push(letter.to_string());
    }
    let mut sum: u128 = 0;
    for letter in letters {
        match letter.as_str() {
            "A" => sum += 1,
            "B" => sum += 2,
            "C" => sum += 3,
            "D" => sum += 4,
            "E" => sum += 5,
            "F" => sum += 6,
            "G" => sum += 7,
            "H" => sum += 8,
            "I" => sum += 9,
            "J" => sum += 10,
            "K" => sum += 11,
            "L" => sum += 12,
            "M" => sum += 13,
            "N" => sum += 14,
            "O" => sum += 15,
            "P" => sum += 16,
            "Q" => sum += 17,
            "R" => sum += 18,
            "S" => sum += 19,
            "T" => sum += 20,
            "U" => sum += 21,
            "V" => sum += 22,
            "W" => sum += 23,
            "X" => sum += 24,
            "Y" => sum += 25,
            "Z" => sum += 26,
            _ => panic!("Invalid letter"),
        }
    }
    sum
}
