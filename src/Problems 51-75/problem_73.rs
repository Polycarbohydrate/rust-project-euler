// https://projecteuler.net/problem=73
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut fractions = vec![];
    for d in 1..=12000 {
        let numerator = (1u64..d).into_iter().collect::<Vec<u64>>();
        for n in numerator {
            if (n as f64 / d as f64 >= 1f64 / 3f64) && (n as f64 / d as f64 <= 1f64 / 2f64) {
                let mut a = d;
                let mut b = n;
                let mut r = a % b;
                let mut check = 0;
                while r != 0 {
                    if r == 1 {
                        check = 1;
                    }
                    a = b;
                    b = r;
                    r = a % b;
                }
                if check == 1 {
                    fractions.push(n as f64 / d as f64);
                }
            }
        }
    }
    let mut counter = 0;
    for fraction in fractions {
        if fraction > (1f64 / 3f64)  && fraction < 1f64 / 2f64 {
            counter += 1;
        }
    }
    println!("Fractions: {}", counter);
    println!("====================================");
    println!("Realtime: {}", now.elapsed().as_secs());
}