// https://projecteuler.net/problem=76
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let numbers = (1..100).collect::<Vec<i32>>();
    let mut list = Vec::new();
    for _ in 0..=100 {
        list.push(0);
    }
    list[0] = 1;
    for num in numbers {
        for x in num..101 {
            list[x as usize] += list[(x - num) as usize];
        }
    }
    println!("The answer is: {}", list[100]);
    println!("====================================");
    println!("{} nanoseconds of runtime", now.elapsed().as_nanos());
}