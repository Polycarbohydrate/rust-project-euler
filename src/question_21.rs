//https://projecteuler.net/problem=21
fn main() {
    let mut sum_of_amicable_numbers: Vec<u128> = Vec::new();
    for number in 1..10_000 {
        let sum_of_perfect_div: u128 = sum_of_divisors(number);
        let sum_of_perfect_div2: u128 = sum_of_divisors(sum_of_perfect_div);
        if sum_of_perfect_div2 == number && sum_of_perfect_div != number {
            if sum_of_amicable_numbers.contains(&number) {
                continue;
            }
            sum_of_amicable_numbers.push(number);
            sum_of_amicable_numbers.push(sum_of_perfect_div);
        }
    }
    let sum: u128 = sum_of_amicable_numbers.iter().sum();
    println!("{sum}");
}

fn sum_of_divisors(number: u128) -> u128 {
    let mut sum: u128 = 0;
    for divisor in 1..number {
        if number % divisor == 0 {
            sum += divisor;
        }
    }
    sum
}
