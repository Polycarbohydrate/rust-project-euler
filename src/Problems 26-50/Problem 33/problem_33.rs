// https://projecteuler.net/problem=33
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut fractions: Vec<(u32, u32)> = vec![];
    for initial_numerator in 10..100 {
        for initial_denominator in initial_numerator+1..100 {
            if initial_numerator % 10 == 0 && initial_denominator % 10 == 0 {
                continue;
            }
            let quotient = initial_numerator as f64 / initial_denominator as f64;
            let num_digits = [initial_numerator / 10, initial_numerator % 10];
            let den_digits = [initial_denominator / 10, initial_denominator % 10];
            for &num_pos in &[0, 1] {
                for &den_pos in &[0, 1] {
                    if num_digits[num_pos] == den_digits[den_pos] && num_digits[num_pos] != 0 {
                        let new_num = num_digits[1 - num_pos];
                        let new_den = den_digits[1 - den_pos];
                        if new_den != 0 && (new_num as f64 / new_den as f64 - quotient).abs() < 1e-10 {
                            fractions.push((initial_numerator, initial_denominator));
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", fractions);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}
