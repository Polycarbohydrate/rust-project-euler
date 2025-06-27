// https://projecteuler.net/problem=54
use std::time::Instant;
use std::fs::read_to_string;
fn main() {
    let now = Instant::now();
    let contents = read_to_string("0054_poker.txt").expect("Something went wrong reading the file");
    let mut player1_hands_won = 0;
    for each_line in contents.lines() {
        let both_hands: Vec<&str> = each_line.split(" ").collect();
        let hand1: Vec<&str> = both_hands[0..5].to_vec();
        let hand2: Vec<&str> = both_hands[5..10].to_vec();
        let mut formatted_hand1 = Vec::new();
        for card in hand1 {
            let rank = card.chars().nth(0).unwrap();
            let suit = card.chars().nth(1).unwrap();
            formatted_hand1.push((rank, suit));
        }
        let mut formatted_hand2 = Vec::new();
        for card in hand2 {
            let rank = card.chars().nth(0).unwrap();
            let suit = card.chars().nth(1).unwrap();
            formatted_hand2.push((rank, suit));
        }
        let high_card1 = check_high_card(&formatted_hand1);
        let high_card2 = check_high_card(&formatted_hand2);
        let check_one_pair1 = check_one_pair(&formatted_hand1);
        let check_one_pair2 = check_one_pair(&formatted_hand2);
        let check_two_pair1 = check_two_pair(&formatted_hand1);
        let check_two_pair2 = check_two_pair(&formatted_hand2);
        let check_threes1 = threes(&formatted_hand1);
        let check_threes2 = threes(&formatted_hand2);
        let check_straight1 = straight(&formatted_hand1);
        let check_straight2 = straight(&formatted_hand2);
        let flush1 = flush(&formatted_hand1);
        let flush2 = flush(&formatted_hand2);
        let mut full_house1 = (false, '0', '0');
        let mut full_house2 = (false, '0', '0');
        if check_one_pair1.1 != false && check_threes1.1 != false && check_one_pair1.2 != check_threes1.2 {
            full_house1 = (true, check_one_pair1.2 , check_threes1.2);
        }
        if check_one_pair2.1 != false && check_threes2.1 != false && check_one_pair2.2 != check_threes2.2 {
            full_house2 = (true, check_one_pair2.2 , check_threes2.2);
        }

        // let check_fours1 = fours(&formatted_hand1);
        // let check_fours2 = fours(&formatted_hand2);
        // let check_straight_flush1 = check_straight1.1 && flush1;
        // let check_straight_flush2 = check_straight2.1 && flush2;
        // let check_royal_flush1 = check_straight_flush1 && check_straight1.2 == 14;
        // let check_royal_flush2 = check_straight_flush2 && check_straight2.2 == 14;

        // There are no royal flushes, straight flushes, or fours in the problem file so we can skip.
        // There are only two full houses, both of which belong to player 2. This doesn't affect player 1's win count.

        if full_house1.0 == true && full_house2.0 == false {
            player1_hands_won += 1;
        } else if full_house1.0 == true && full_house2.0 == true {
            println!("Tie full houses detected");
        }
        if flush1 == true && flush2 == false && full_house2.0 == false && full_house1.0 == false {
            player1_hands_won += 1;
        } else if flush1 == true && flush2 == true && full_house2.0 == false && full_house1.0 == false{
            println!("Tie flushes detected");
        }
        if check_straight1.1 == true && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            player1_hands_won += 1;
        } else if check_straight1.1 == true && check_straight2.1 == true
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            println!("Tie straights detected");
        }
        if check_threes1.1 == true && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            player1_hands_won += 1;
        } else if check_threes1.1 == true && check_threes2.1 == true
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            println!("Tie threes detected");
        }
        if check_two_pair1.1 == true && check_two_pair2.1 == false
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            player1_hands_won += 1;
        } else if check_two_pair1.1 == true && check_two_pair2.1 == true
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            println!("Tie two pairs detected");
        }
        if check_one_pair1.1 == true && check_one_pair2.1 == false
            && check_two_pair1.1 == false
            && check_two_pair2.1 == false
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            player1_hands_won += 1;
        } else if check_one_pair1.1 == true && check_one_pair2.1 == true
            && check_two_pair1.1 == false
            && check_two_pair2.1 == false
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            let one = convert_face_cards_to_num(check_one_pair1.2);
            let two = convert_face_cards_to_num(check_one_pair2.2);
            if one > two {
                player1_hands_won += 1;
            } else if one == two {
                println!("Tie one pairs with same value detected");
                println!("Player 2 wins this hand");
            }
        }
        if high_card1.1 > high_card2.1
            && check_one_pair1.1 == false
            && check_one_pair2.1 == false
            && check_two_pair1.1 == false
            && check_two_pair2.1 == false
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            player1_hands_won += 1;
        } else if high_card1.1 == high_card2.1  && check_one_pair1.1 == false
            && check_one_pair2.1 == false
            && check_two_pair1.1 == false
            && check_two_pair2.1 == false
            && check_threes1.1 == false
            && check_threes2.1 == false
            && check_straight1.1 == false
            && check_straight2.1 == false
            && flush1 == false
            && flush2 == false
            && full_house2.0 == false
            && full_house1.0 == false {
            println!("Tie high cards with same value detected");
        }
    }
    println!("Player 1 hands won: {}", player1_hands_won);
    println!("====================================");
    println!("Real time elapsed: {}", now.elapsed().as_secs());
}

fn convert_face_cards_to_num(card: char) -> u8 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card"),
    }
}

fn check_high_card(hand: &Vec<(char, char)>) -> (char, u8) {
    let mut high_card = ('2', 2);
    for card in hand.iter() {
        let card_value = convert_face_cards_to_num(card.0);
        let high_card_value = convert_face_cards_to_num(high_card.0);
        if card_value >= high_card_value {
            high_card = (card.0, card_value);
        }
    }
    high_card
}

fn check_one_pair(hand: &Vec<(char, char)>) -> (String ,bool, char) {
    let mut checker = hand.clone();
    checker.sort();
    let mut value = (String::from("one_pair"), false, '0');
    let mut compare_possible_values = Vec::new();
    for index in 0..checker.len() - 1 {
        let first_card = checker[index].0;
        let second_card = checker[index + 1].0;
        if first_card == second_card {
            compare_possible_values.push(first_card);
        }
    }
    if compare_possible_values.len() == 1 {
        value = (String::from("one_pair"), true, compare_possible_values[0]);
        value
    } else if compare_possible_values.len() == 0 {
        return value
    } else {
        let mut value_with_numerical_value = Vec::new();
        for card in compare_possible_values {
            let num_value = convert_face_cards_to_num(card);
            value_with_numerical_value.push((card, num_value));
        }
        value_with_numerical_value.sort_by(|a, b| b.1.cmp(&a.1));
        value = (String::from("one_pair"), true, value_with_numerical_value[0].0);
        value
    }
}

fn check_two_pair(hand: &Vec<(char, char)>) -> (String, bool, char, char) {
    let mut checker = hand.clone();
    checker.sort();
    let mut value = (String::from("two_pair"), false, '0', '0');
    let mut compare_possible_values = Vec::new();
    for index in 0..checker.len() - 1 {
        let first_card = checker[index].0;
        let second_card = checker[index + 1].0;
        if first_card == second_card {
            compare_possible_values.push(first_card);
        }
    }
    compare_possible_values.sort();
    compare_possible_values.dedup();
    if compare_possible_values.len() == 2 {
        value = (String::from("two_pair"), true, compare_possible_values[0], compare_possible_values[1]);
        value
    } else {
        value
    }
}

fn threes(hand: &Vec<(char, char)>) -> (String, bool, char) {
    let mut checker = hand.clone();
    checker.sort();
    let mut value = (String::from("threes"), false, '0');
    for index in 0..checker.len() - 2 {
        let first_card = checker[index].0;
        let second_card = checker[index + 1].0;
        let third_card = checker[index + 2].0;
        if first_card == second_card && first_card == third_card {
            value.1 = true;
            value.2 = first_card;
        }
    }
    value
}

fn straight(hand: &Vec<(char, char)>) -> (String, bool, u8) {
    let checker = hand.clone();
    let mut value = (String::from("straight"), false, 0);
    let mut numerical_values = Vec::new();
    for card in checker.clone() {
        let value = convert_face_cards_to_num(card.0);
        numerical_values.push(value);
    }
    numerical_values.sort();
    let mut count = 0;
    for index in 0..numerical_values.len() - 1 {
        if numerical_values[index + 1] - numerical_values[index] == 1 {
            count += 1;
        }
    }
    if count == 4 {
        value.1 = true;
        value.2 = numerical_values[4];
    }
    value
}

fn flush(hand: &Vec<(char, char)>) -> bool {
    let base_suit = hand[0].1;
    let mut value = true;
    for card in hand {
        if base_suit != card.1 {
            value = false;
        }
    }
    value
}

// fn fours(hand: &Vec<(char, char)>) -> (String, bool, char) {
//     let mut checker = hand.clone();
//     checker.sort();
//     let mut value = (String::from("threes"), false, '0');
//     for index in 0..checker.len() - 3 {
//         let first_card = checker[index].0;
//         let second_card = checker[index + 1].0;
//         let third_card = checker[index + 2].0;
//         let fourth_card = checker[index + 3].0;
//         if first_card == second_card && first_card == third_card && first_card == fourth_card {
//             value.1 = true;
//             value.2 = first_card;
//         }
//     }
//     value
// }