// https://projecteuler.net/problem=7
fn main()   {
    let input: u128 = 110_000;
    let sqrt_input = (input as f64).sqrt() as u128;
    let mut a : Vec<(bool, u128)> = Vec::new();
    for i in 2..input {
        a.push((true, i));
    }
    for i in 2..=sqrt_input  {
        if a[(i - 2) as usize].0 {
            let remove = j_calculations(i);
            for j in remove {
                if (j as usize) < a.len() {
                    a[(j - 2) as usize].0 = false;
                }
            }
        };
    }
    a.retain(|&(x, _)| x);
    println!("{:#?}", a[10_000]);
}

fn j_calculations(input: u128) -> Vec<u128> {
    let i = input;
    let mut false_js: Vec<u128> = Vec::new();
    let mut j = i;
    while i * j < 110_000 {
        false_js.push(i * j);
        j += 1;
    }
    false_js
}
