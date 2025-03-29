// https://projecteuler.net/problem=2
fn main() {
    let mut sum: u128 = 0;
    let mut a: u128 = 1;
    let mut b: u128 = 2;

    while b <= 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    println!("{}",sum);
}
