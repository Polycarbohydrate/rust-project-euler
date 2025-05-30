// https://projecteuler.net/problem=20
//
// Used Python to calculate the numbers due to Rust's integer limitations.
//
// x = 1
// for i in range(1, 101):
//     x = x * i
// print(x)

print(x)
fn main() {
    let mut numbers = String::from("93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000");
    let sum: Vec<u128> = numbers.chars().map(|c| c.to_digit(10).unwrap() as u128).collect();
    println!("{}", sum.iter().sum::<u128>());
    
}
