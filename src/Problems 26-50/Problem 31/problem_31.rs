// https://projecteuler.net/problem=31
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut sum: u64 = 0;
    for one_pence in 0..=200 {
        for two_pence in 0..=100 {
            for five_pence in 0..=40 {
                for ten_pence in 0..=20 {
                    for twenty_pence in 0..=10 {
                        for fifty_pence in 0..=4 {
                            for one_pound in 0..=2 {
                                for two_pound in 0..=2 {
                                    let total = one_pence * 1
                                        + two_pence * 2
                                        + five_pence * 5
                                        + ten_pence * 10
                                        + twenty_pence * 20
                                        + fifty_pence * 50
                                        + one_pound * 100
                                        + two_pound * 200;
                                    if total == 200 {
                                        sum += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum);
    println!("{}", now.elapsed().as_millis());
}