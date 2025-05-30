// https://projecteuler.net/problem=12
fn main() {
    let mut answer = Vec::new();
    for i in 1..1_000_000   {
        let triangular_num = i * (i + 1) / 2;
        let mut factors = Vec::new();
        for i in 1..=(triangular_num as f64).sqrt() as u32 {
            if triangular_num % i == 0 {
                factors.push(i);
                if i != triangular_num / i {
                    factors.push(triangular_num / i);
                }
            }
        }
        if factors.len() > 500 {
            answer.push(triangular_num);
            break;
        }
    }
    println!("{:?}", answer);
}
