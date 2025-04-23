// https://projecteuler.net/problem=17
fn main() {
    let first_set = vec!["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let second_set = vec!["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let third_set = vec!["hundred", "thousand", "and"];
    let mut characters = 0;
    for num in 1..=1000 {
        if num < 20 {
            let chars = first_set[num].len();
            characters += chars;
        } else if num < 100 {
            let tens = num / 10;
            let ones = num % 10;
            let chars = second_set[tens].len() + first_set[ones].len();
            characters += chars;
        } else if num < 1000 {
            let hundreds = num / 100;
            let remainder = num % 100;
            characters += first_set[hundreds].len() + third_set[0].len();
            if remainder > 0 {
                characters += third_set[2].len();
                if remainder < 20 {
                    characters += first_set[remainder].len();
                } else {
                    let tens = remainder / 10;
                    let ones = remainder % 10;
                    characters += second_set[tens].len() + first_set[ones].len();
                }
            }
        } else if num == 1000 {
            let chars = first_set[1].len() + third_set[1].len();
            characters += chars;
        } else {
            println!("Failure: {}", num);
        }
    }
    println!("{}", characters);
}
