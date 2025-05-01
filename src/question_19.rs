// https://projecteuler.net/problem=19
fn main()   {
    let mut total_number_of_sundays = 0;
    let days = vec![1, 2, 3, 4, 5, 6, 7];
    let months = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut starting_day = days[1];
    for year in 1901..=2000 {
        if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
            for month in months.iter() {
                if month == &2 {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=29 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                } else if month == &4 || month == &6 || month == &9 || month == &11 {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=30 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                } else {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=31 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                }
            }
        } else {
            for month in months.iter() {
                if month == &2 {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=28 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                } else if month == &4 || month == &6 || month == &9 || month == &11 {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=30 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                } else {
                    let initial_day = starting_day;
                    let mut days_counter = 0;
                    for _ in 1..=31 {
                        days_counter += 1;
                        if days_counter == 1 {
                            if initial_day == 7 {
                                total_number_of_sundays += 1;
                            }
                        }
                    }
                    let new_starting_day = (days_counter + initial_day) % 7;
                    if new_starting_day == 0 {
                        starting_day = days[6];
                    } else {
                        starting_day = days[new_starting_day - 1];
                    }
                }
            }
        }
    }
    println!("Total number of Sundays that fell on the first of the month during the twentieth century: {}", total_number_of_sundays);
}
