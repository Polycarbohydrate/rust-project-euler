// https://projecteuler.net/problem=28
use std::time::Instant;
fn main() {
    let start_time = Instant::now();
    let mut total_numbers: Vec<u64> = Vec::new();
    for number in 2..=1002001 {
        total_numbers.push(number);
    }
    let mut sum: u128 = 1;
    for n in 1..=500 {
        let side_size = (n * 2) + 1;
        let total_area = side_size * side_size;
        let small_ring_size = n-1;
        let small_side_size = (small_ring_size * 2) + 1;
        let small_area = small_side_size * small_side_size;
        let numbers_in_ring = total_area - small_area;
        let mut ring_numbers = Vec::new();
        for number in 0..numbers_in_ring {
            ring_numbers.push(total_numbers[number]);
        }
        for _ in 0..numbers_in_ring {
            total_numbers.remove(0);
        }
        let chunk_size = numbers_in_ring / 4;
        let spliced = ring_numbers.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect::<Vec<_>>();
        for vec in &spliced {
            sum += *vec.last().unwrap() as u128;
        }
    }
    println!("{sum}");
    println!("{}", start_time.elapsed().as_secs());
}