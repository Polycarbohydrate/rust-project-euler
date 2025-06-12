// https://projecteuler.net/problem=52
use std::time::Instant;
fn main() {
    let now = Instant::now();
    for num in 1..1_000_000 {
        let mut master_num_chars = num.to_string().chars().collect::<Vec<char>>();
        let num_times_two = num * 2;
        let mut num_times_two_chars = num_times_two.to_string().chars().collect::<Vec<char>>();
        let num_times_three = num * 3;
        let mut num_times_three_chars = num_times_three.to_string().chars().collect::<Vec<char>>();
        let num_times_four = num * 4;
        let mut num_times_four_chars = num_times_four.to_string().chars().collect::<Vec<char>>();
        let num_times_five = num * 5;
        let mut num_times_five_chars = num_times_five.to_string().chars().collect::<Vec<char>>();
        let num_times_six = num * 6;
        let mut num_times_six_chars = num_times_six.to_string().chars().collect::<Vec<char>>();
        master_num_chars.sort();
        num_times_two_chars.sort();
        num_times_three_chars.sort();
        num_times_four_chars.sort();
        num_times_five_chars.sort();
        num_times_six_chars.sort();
        if master_num_chars == num_times_two_chars
            && master_num_chars == num_times_three_chars
            && master_num_chars == num_times_four_chars
            && master_num_chars == num_times_five_chars
            && master_num_chars == num_times_six_chars
        {
            println!("The number is: {}", num);
            println!("The multiples are: {}, {}, {}, {}, {}", 
                num_times_two, num_times_three, num_times_four, num_times_five, num_times_six);
            break;
        }
    }
    println!("================================");
    println!("{} seconds of runtime", now.elapsed().as_secs());
}