// https://projecteuler.net/problem=3
fn main() {
    let n: u128 = 600851475143;
    let largest = find_largest_prime_factor(n);
    println!("The largest prime factor is: {}", largest);
}
fn find_largest_prime_factor(mut n: u128) -> u128 {
    let mut largest = 1;
    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }
    while n > 1 {
        if is_prime(n) {
            return n;
        }
        let factor = pollards_rho(n).unwrap_or(n);
        n /= factor;
        largest = largest.max(factor);
    }
    largest
}
fn pollards_rho(n: u128) -> Option<u128> {
    if n % 2 == 0 {
        return Some(2);
    }

    let mut x: u128 = 2;
    let mut y: u128 = 2;
    let mut d: u128 = 1;

    while d == 1 {
        x = gx(x, n);
        y = gx(gx(y, n), n);
        let diff = if x > y { x - y } else { y - x };
        d = gcd(diff, n);
    }

    if d == n {
        None
    } else {
        Some(d)
    }
}
fn gx(input: u128, n: u128) -> u128 {
    ((input * input) + 1) % n
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
