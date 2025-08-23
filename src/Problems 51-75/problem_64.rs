// https://projecteuler.net/problem=64
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut count = 0;
    for d in 2..=10_000i32 {
        let sqrt = (d as f64).sqrt();
        if sqrt.fract() == 0.0 {
            continue;
        }
        let r = d.isqrt();
        let mut a = d.isqrt();
        let mut p = 0;
        let mut q = 1;
        let mut period = 0;
        while a != 2 * r {
            p = a * q - p;
            q = (d - p * p) / q;
            a = (r + p) / q;
            period += 1;
        }
        if period % 2 != 0 {
            count += 1;
        }
    }
    println!("Answer: {}", count);
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}