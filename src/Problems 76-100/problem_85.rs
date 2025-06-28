// https://projecteuler.net/problem=85
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut width = 2;
    let mut height = 3;
    let mut num_of_rectangles = 0;
    let mut min_diff = 2_000_000;
    for width1 in 1..1000 {
        for height1 in 1..1000 {
            let h = width1 + 1;
            let v = height1 + 1;
            let rectangles: i64 = (h * v) * (h - 1) * (v - 1) / 4;
            let diff = (rectangles - 2_000_000).abs();
            if diff < min_diff {
                min_diff = diff;
                num_of_rectangles = rectangles;
                width = width1;
                height = height1;
            }
        }
    }
    let area = width * height;
    println!("Area: {}", area);
    println!("Rectangles: {}", num_of_rectangles);
    println!("Width: {}, Height: {}", width, height);
    println!("====================================");
    println!("Real time elapsed: {}", now.elapsed().as_secs());
}