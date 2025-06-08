// https://projecteuler.net/problem=38
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut pandigital: u64 = 0;
    for initial_integer in 1..500000 {
        let mut output: Vec<char> = multiplication(initial_integer);
        let temp = output.clone();
        let x = temp.iter().collect::<String>();
        let y = x.parse::<u64>().unwrap();
        output.sort();
        if output.len() == 9 {
            if output == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                if y > pandigital {
                    pandigital = y;
                    println!("Pandigital: {}", pandigital);
                }
            }
        }
    }
    println!("{pandigital}");
    println!("================================");
    println!("{} milliseconds of runtime", now.elapsed().as_millis());
}

fn multiplication(int: i32) -> Vec<char> {
    let mut multiplier = 1;
    let mut result: Vec<char> = Vec::new();
    while result.len() < 9 {
        let product = int * multiplier;
        let product_str = product.to_string();
        for c in product_str.chars() {
            result.push(c);
        }
        if result.len() > 9 {
            break
        }
        multiplier += 1;
    }
    if multiplier > 2 {
        result
    } else {
        vec!['0']
    }
}