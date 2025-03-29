fn main() {
    let sum: i32 = (0..1000)
        .filter(|num| num % 3 == 0 || num % 5 == 0)
        .sum();
    println!("{}", sum);
}
