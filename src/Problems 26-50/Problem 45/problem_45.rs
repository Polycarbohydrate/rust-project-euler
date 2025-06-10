// https://projecteuler.net/problem=45
use std::time::Instant;
fn main() {
    let now = Instant::now();
    for n in 286.. {
        let t = n * (n + 1) / 2;
        if check_hexagonal(t) && check_pentagonal(t) {
            println!("The next triangular number that is also pentagonal and hexagonal is: {}", t);
            break;
        }
    }
    println!("================================");
    println!("{} nanoseconds of runtime", now.elapsed().as_nanos());
}

fn check_pentagonal(n: u64) -> bool {
    let p = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    p == p.floor()
}

fn check_hexagonal(n: u64) -> bool {
    let h = (1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 4.0;
    h == h.floor()
}