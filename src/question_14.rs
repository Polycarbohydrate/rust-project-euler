// https://projecteuler.net/problem=14
use std::collections::HashMap;
fn main() {
    let mut data: HashMap<u128, u128> = HashMap::new();
    for i in 1..1_000_000 {
        let mut n = i;
        let mut counter = 0;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
                counter += 1;
            } else {
                n = 3 * n + 1;
                counter += 1;
            }
        }
        data.insert(counter, i);
    }
    let mut sorted_data: Vec<(&u128, &u128)> = data.iter().collect();
    sorted_data.sort_by(|a, b| a.0.cmp(b.0));
    println!("Data is sorted: <Counter, Number>");
    println!("{:#?}", sorted_data);
}
