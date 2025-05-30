// https://projecteuler.net/problem=5
fn main() {
    let mut nums = Vec::new();
    for i in 1..1000000000    {
        check_divisible_by_nums(i);
        if check_divisible_by_nums(i) {
            nums.push(i);
        }
    }
    nums.sort();
    println!("{:#?}", nums);
}

fn check_divisible_by_nums(num: i32) -> bool {
    for i in 1..=20 {
        if num % i != 0 {
            return false;
        }
    }
    true
}
