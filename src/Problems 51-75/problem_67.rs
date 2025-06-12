// https://projecteuler.net/problem=67
use std::fs;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let triangle = fs::read_to_string("0067_triangle.txt").unwrap();
    let mut rows_left = 98;
    let last_row = triangle.lines().last().unwrap();
    let mut second_last_row = triangle.lines().nth(rows_left).unwrap();
    let mut numbers_in_second_last_row: Vec<u64> = second_last_row
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut numbers_in_last_row: Vec<u64> = last_row
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    loop {
        for number in 0..numbers_in_second_last_row.len() {
            let left = numbers_in_last_row[number];
            let right = numbers_in_last_row[number + 1];
            let max = if left > right { left } else { right };
            numbers_in_second_last_row[number] += max;
        }
        numbers_in_last_row = numbers_in_second_last_row.clone();
        rows_left -= 1;
        second_last_row = triangle.lines().nth(rows_left).unwrap();
        numbers_in_second_last_row = second_last_row
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        if rows_left == 0 {
            println!("{:?}", numbers_in_second_last_row);
            println!("{:?}", numbers_in_last_row);
            println!("7214 + 59 = 7273");
            break;
        }
    }
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}