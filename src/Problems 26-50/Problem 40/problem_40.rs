// https://projecteuler.net/problem=40
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut storage = Vec::new();
    for number in 1..1_000_000 {
        let x = number.to_string().chars().collect::<Vec<_>>();
        for char in x {
            storage.push(char);
        }
    }
    let product = storage[0].to_digit(10).unwrap()
        * storage[9].to_digit(10).unwrap()
        * storage[99].to_digit(10).unwrap()
        * storage[999].to_digit(10).unwrap()
        * storage[9999].to_digit(10).unwrap()
        * storage[99999].to_digit(10).unwrap();
    println!("Length of storage: {}", storage.len());
    println!("Product of digits: {}", product);
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}
