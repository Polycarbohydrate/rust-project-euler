// https://projecteuler.net/problem=71
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let mut new_numerator = 1;
    let mut new_denominator = 1;
    let mut new_ratio = 1.0;
    for denominator in 990_000..=1_000_000 {
        for numerator in 2..(denominator / 2) {
            let mut is_simplified = false;
            let mut remainder = 0;
            if denominator % numerator != 0 {
                let mut a = denominator;
                let mut b = numerator;
                let mut q = a / b;
                let mut r = a - (b * q);
                while r != 0 {
                    if r != 0 {
                        remainder = r;
                    }
                    a = b;
                    b = r;
                    q = a / b;
                    r = a - (b * q);
                }
                if remainder == 1 {
                    is_simplified = true;
                }
                if is_simplified {
                    let current_ratio = numerator as f64 / denominator as f64;
                    let threshold_ratio = 3.0 / 7.0;
                    if new_denominator == 1 && new_numerator == 1 {
                        new_numerator = numerator;
                        new_denominator = denominator;
                        new_ratio = current_ratio;
                    }
                    if current_ratio < threshold_ratio {
                        if current_ratio > new_ratio  {
                            new_numerator = numerator;
                            new_denominator = denominator;
                            new_ratio = current_ratio;
                        }
                    }
                }
            }
        }
    }
    println!("The numerator is {}", new_numerator);
    println!("The denominator is {}", new_denominator);
    println!("The ratio is {}", new_ratio);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
    fs::write("output.txt", format!("Final Numerator: {}\nFinal Denominator: {}\nFinal Ratio: {}\n", new_numerator, new_denominator, new_ratio)).expect("Unable to write to file");
}