// https://projecteuler.net/problem=24
fn main() {
    let digits = (0..10).collect::<Vec<_>>();
    let millionth = nth_lexicographic_permutation(digits, 1_000_000);
    println!("Millionth permutation: {:?}", millionth);
}

fn nth_lexicographic_permutation(mut items: Vec<usize>, mut n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut factorial = (1..=items.len()).scan(1, |state, x| {
        *state *= x;
        Some(*state)
    }).collect::<Vec<_>>();
    factorial.insert(0, 1);
    n -= 1;
    for i in (0..items.len()).rev() {
        let f = factorial[i];
        let index = n / f;
        result.push(items.remove(index));
        n %= f;
    }
    result
}
