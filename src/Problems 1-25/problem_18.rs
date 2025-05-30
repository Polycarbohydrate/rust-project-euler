// https://projecteuler.net/problem=18
fn main() {
    let triangle = String::from("75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23");
    let mut line_number = 13;
    let mut new_line_nums: Vec<u32> = Vec::new();
    let mut new_directions: Vec<String> = Vec::new();
    println!("Triangle line count: {}", triangle.lines().count());
    loop {
        if line_number == 13 {
            let second_last_line = triangle.lines().nth(line_number).unwrap().to_string();
            let second_last_row: Vec<u32> = second_last_line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            let last_line = triangle.lines().nth(line_number + 1).unwrap().to_string();
            let last_line_nums: Vec<u32> = last_line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            let mut l = 0;
            let mut r = 1;
            while r < last_line_nums.len() {
                for number in &second_last_row {
                    let left_sum = number + last_line_nums[l];
                    let right_sum = number + last_line_nums[r];
                    if left_sum > right_sum {
                        let num = left_sum;
                        let direction = "L".to_string();
                        new_line_nums.push(num);
                        new_directions.push(direction);
                    } else if right_sum > left_sum {
                        let num = right_sum;
                        let direction = "R".to_string();
                        new_line_nums.push(num);
                        new_directions.push(direction);
                    } else {
                        println!("Left and right are equal: {left_sum} = {right_sum}");
                    }
                    l += 1;
                    r += 1;
                }
            }
            line_number -= 1;
        } else if line_number > 0 {
            let second_last_line = triangle.lines().nth(line_number).unwrap().to_string();
            let second_last_row: Vec<u32> = second_last_line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            let last_line_nums: Vec<u32> = new_line_nums.clone();
            let mut l = 0;
            let mut r = 1;
            while r < last_line_nums.len() {
                let item_count_before = new_line_nums.len();
                for number in &second_last_row {
                    let left_sum = number + last_line_nums[l];
                    let right_sum = number + last_line_nums[r];
                    if left_sum > right_sum {
                        let num = left_sum;
                        let direction_fetch = new_directions[l].clone();
                        let direction = format!("{direction_fetch}L");
                        new_line_nums.push(num);
                        new_directions.push(direction);
                    } else if right_sum > left_sum {
                        let num = right_sum;
                        let direction_fetch = new_directions[l].clone();
                        let direction = format!("{direction_fetch}R");
                        new_line_nums.push(num);
                        new_directions.push(direction);
                    } else {
                        println!("Left and right are equal: {left_sum} = {right_sum}");
                    }
                    l += 1;
                    r += 1;
                }
                for _ in 0..item_count_before {
                    new_line_nums.remove(0);
                    new_directions.remove(0);
                }
            }
            line_number -= 1;
        } else {
            break;
        }
    }
    println!("Final 2 numbers: {} {}", new_line_nums[0], new_line_nums[1]);
    println!("Initial number in the triangle: {:?}", triangle.lines().nth(0).iter().nth(0).unwrap());
    let initial = triangle.lines().nth(0).iter().nth(0).unwrap().to_string();
    if new_line_nums[0] > new_line_nums[1] {
        let final_sum = new_line_nums[0] + initial.parse::<u32>().unwrap();
        println!("Final sum: {final_sum}");
    } else {
        let final_sum = new_line_nums[1] + initial.parse::<u32>().unwrap();
        println!("Final sum: {final_sum}");
    }
}
