// https://projecteuler.net/problem=10
fn main()   {
    let input:u128 = 2_000_000;
    let input_sqrt = (input as f64).sqrt() as u128;
    let mut a : Vec<(bool, u128)> = Vec::new();
    for i in 2..input {
        a.push((true, i));
    }
    for i in 2..=input_sqrt  {
        if a[(i - 2) as usize].0 {
            let remove = j_calculations(i, input);
            for j in remove {
                if j >=2 && (j as usize) < a.len() + 2 {
                    a[(j - 2) as usize].0 = false;
                }
            }
        }
    }
    a.retain(|&(x, _)| x);
    let sum : u128 = a.iter().map(|(_, y)| y).sum();
    println!("{}", sum);
}

fn j_calculations(input: u128, limit: u128) -> Vec<u128> {
    let i = input;
    let mut false_js: Vec<u128> = Vec::new();
    let mut j = i;
    while i * j < limit {
        false_js.push(i * j);
        j += 1;
    }
    false_js
}
