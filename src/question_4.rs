// https://projecteuler.net/problem=4
fn main() {
    let mut palindromes: Vec<u128> = Vec::new();
    for first in 100..1000 {
        for second in 100..1000 {
            let product = first * second;
            let product_str = product.to_string();
            let reversed: String = product_str.chars().rev().collect();
            if product_str == reversed {
                palindromes.push(product);
            }
        }
    }
    palindromes.sort();
    println!("{:#?}", palindromes);
}
