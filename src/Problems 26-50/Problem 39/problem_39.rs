// https://projecteuler.net/problem=39
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut max_solutions = 0;
    let mut best_p = 0;
    for p in 1..=1000 {
        let mut relative_solutions = 0;
        for a in 1..=p {
            for b in 1..=p {
                let c = (((a as u128).pow(2) + (b as u128).pow(2)) as f64).sqrt() as u128;
                let sum = a + b + c;
                if sum == p && c * c == a * a + b * b {
                    relative_solutions += 1;
                }
            }
        }
        if relative_solutions > max_solutions {
            max_solutions = relative_solutions;
            best_p = p;
        }
    }
    println!("max solutions: {}", max_solutions);
    println!("best p: {}", best_p);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}