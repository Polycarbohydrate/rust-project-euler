// https://projecteuler.net/problem=44
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut difference = 0;
    let mut num1 = 0;
    let mut num2 = 0;
    for n_out in 1..10_000 {
        let outside_pentagonal: i128 = (n_out * ((3 * n_out) - 1)) / 2;
        for n_in in 1..10_000 {
            let inside_pentagonal: i128 = (n_in * ((3 * n_in) - 1)) / 2;
            let sum = outside_pentagonal + inside_pentagonal;
            let diff: i128 = outside_pentagonal - inside_pentagonal;
            if check_is_pentagonal(sum) && check_is_pentagonal(diff) {
                if difference == 0 || diff.abs() < difference {
                    difference = diff.abs();
                    num1 = outside_pentagonal;
                    num2 = inside_pentagonal;
                }
            }
        }
    }
    println!("Smallest difference found: {}", difference);
    println!("Numbers: {} and {}", num1, num2);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}

fn check_is_pentagonal(n: i128) -> bool {
    let x = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    x == x.floor()
}
