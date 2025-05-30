// https://projecteuler.net/problem=15
fn main() {
    let k = 20;
    let r = 20;
    let n = k + r;
    let total_paths = binomial_coefficient(n, k);
    println!("{}", total_paths);
}

fn binomial_coefficient(n: u128, k: u128) -> u128 {
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
