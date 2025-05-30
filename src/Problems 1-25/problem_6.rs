// https://projecteuler.net/problem=7
fn main()   {
    let mut sum1: Vec<i64> = Vec::new();
    for i in 1..=100    {
        sum1.push(i * i);
    }
    let sum1final = sum1.iter().sum::<i64>();
    let mut sum2: Vec<i64> = Vec::new();
    for i in 1..=100    {
        sum2.push(i);
    }
    let sum2final = sum2.iter().sum::<i64>();
    let sum2finalv2 = sum2final * sum2final;
    let final_sum = sum2finalv2 - sum1final;
    println!("{}", final_sum);
}
