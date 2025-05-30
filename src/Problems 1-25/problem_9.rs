// https://projecteuler.net/problem=9
fn main() {
    for m in 2..1000 {
        for n in 1..m {
            if check_is_coprime(n, m) && check_one_is_even(n, m) {
                let a = m.pow(2) - n.pow(2);
                let b = 2 * m * n;
                let c = m.pow(2) + n.pow(2);
                println!("a: {}, b: {}, c: {}", a, b, c);
                for k in 1..1000 {
                    let ka = k * a;
                    let kb = k * b;
                    let kc = k * c;
                    if ka + kb + kc == 1000 {
                        println!("{} {} {}", ka, kb, kc);
                        let product = ka * kb * kc;
                        println!("{}", product);
                        return;
                    } else if ka + kb + kc > 1000 {
                        break;
                    }
                }
            }
        }
    }
}

fn check_is_coprime(mut x: u128, mut y: u128) -> bool   {
    while y != 0 {
        let z = y;
        y = x % y;
        x = z;
    }
    x == 1
}

fn check_one_is_even(x: u128, y: u128) -> bool  {
    if x % 2 == 0 {
        y % 2 != 0
    }   else {
        y % 2 == 0
    }
}
