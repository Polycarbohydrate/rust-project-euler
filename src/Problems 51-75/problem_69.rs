// https://projecteuler.net/problem=69
use std::time::Instant;
use std::fs;
fn main() {
    let now = Instant::now();
    let mut maximum: f64 = 0.0;
    let mut n = 0;
    for number in 1..=1_000_000 {
        if number % 10_000 == 0 {
            let contents = format!("Number: {}\nMaximum quotient: {}\nCurrent number: {}", n, maximum, number);
            fs::write("output.txt", contents).unwrap();
            println!("{}", number);
            println!("{} seconds of runtime", now.elapsed().as_secs());
            println!("================================");
        }
        let mut count = 1;
        for test_outside in 2..=number {
            let mut remainder = 0;
            if number % test_outside != 0 {
                let mut a = if number > test_outside { number } else { test_outside };
                let mut b = if number < test_outside { number } else { test_outside };
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
                    count += 1;
                }
            }
        }
        let quotient = number as f64 / count as f64;
        if quotient > maximum {
            maximum = quotient;
            n = number;
        }
    }
    println!("================================");
    println!("Maximum quotient: {}", maximum);
    println!("Number with maximum quotient: {}", n);
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
    let contents2 = format!("Maximum quotient: {} \nNumber with maximum quotient: {}", maximum, n);
    fs::write("output.txt", contents2).unwrap();
}