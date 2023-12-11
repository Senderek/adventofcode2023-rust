use crate::utils;

#[allow(dead_code)]
pub fn solve_7a() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input7.txt").unwrap();

    let inputs: Vec<_> = lines
        .map(|l| {
            if let Ok(line) = l {
                let (hand, bid) = line.split_once(' ').unwrap();
                return (hand.to_owned(), bid.parse::<u32>().unwrap());
            } else {
                panic!();
            }
        })
        .collect();

    println!("{:?}", inputs);

    let mut sorted_by_rank = inputs.clone();
    sorted_by_rank.sort_by(|(hand1, _), (hand2, _)| {
        let val_x = map_hand_to_value(hand1);
        let val_y = map_hand_to_value(hand2);
        let r = val_x.cmp(&val_y);
        if r == std::cmp::Ordering::Equal {
            let o = hand1.chars().enumerate().find_map(|(i, c)| {
                let c_val = map_char_to_value(&c);
                let c2_chars = hand2.chars().collect::<Vec<_>>();
                let c2 = c2_chars.get(i).unwrap();
                let c2_val = map_char_to_value(c2);
                let cmp = c_val.cmp(&c2_val);
                if cmp != std::cmp::Ordering::Equal {
                    return Some(cmp);
                }

                return None;
            });

            return match o {
                Some(ord) => ord,
                _ => std::cmp::Ordering::Equal,
            };
        } else {
            return r;
        }
    });

    println!("{:?}", sorted_by_rank);

    let result = sorted_by_rank
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| {
            let rank = u32::try_from(i + 1).unwrap();
            return *(bid) * rank;
        })
        .sum::<u32>();

    println!("Result: {}", result);
}

fn map_hand_to_value(x: &String) -> u32 {
    let mut chars = x.clone().chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    let count = chars.len();
    if count == 5 {
        // println!("High card: {}", x);
        return 1;
    } else if count == 1 {
        // println!("Five of a kind: {}", x);
        return 7;
    } else if count == 2 {
        let first_letter_count = x.chars().filter(|x| x == chars.get(0).unwrap()).count();
        if first_letter_count == 1 || first_letter_count == 4 {
            // println!("Four of a kind: {}", x);
            return 6;
        } else {
            // println!("Full house: {}", x);
            return 5;
        }
    } else if count == 4 {
        // println!("Pair: {}", x);
        return 2;
    } else {
        if chars
            .iter()
            .map(|c| x.chars().filter(|x| x == c).count())
            .any(|x| x == 3)
        {
            // println!("Three of a kind: {}", x);
            return 4;
        }
        // println!("Two pairs: {}", x);
        return 3;
    }
}

fn map_char_to_value(x: &char) -> u32 {
    const RADIX: u32 = 10;
    return match x.to_digit(RADIX) {
        Some(it) => it,
        _ => match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => panic!("{}", x),
        },
    };
}

#[allow(dead_code)]
pub fn solve_7b() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input7.txt").unwrap();

    let inputs: Vec<_> = lines
        .map(|l| {
            if let Ok(line) = l {
                let (hand, bid) = line.split_once(' ').unwrap();
                return (hand.to_owned(), bid.parse::<u32>().unwrap());
            } else {
                panic!();
            }
        })
        .collect();

    for (h, _) in &inputs {
        if h.contains('J') {
            println!("{}: {}", h, map_hand_to_value_with_joker(h));
        }
    }

    let mut sorted_by_rank = inputs.clone();
    sorted_by_rank.sort_by(|(hand1, _), (hand2, _)| {
        let val_x = map_hand_to_value_with_joker(hand1);
        let val_y = map_hand_to_value_with_joker(hand2);
        let r = val_x.cmp(&val_y);
        if r == std::cmp::Ordering::Equal {
            let o = hand1.chars().enumerate().find_map(|(i, c)| {
                let c_val = map_char_to_value_with_joker(&c);
                let c2_chars = hand2.chars().collect::<Vec<_>>();
                let c2 = c2_chars.get(i).unwrap();
                let c2_val = map_char_to_value_with_joker(c2);
                let cmp = c_val.cmp(&c2_val);
                if cmp != std::cmp::Ordering::Equal {
                    return Some(cmp);
                }

                return None;
            });

            return match o {
                Some(ord) => ord,
                _ => std::cmp::Ordering::Equal,
            };
        } else {
            return r;
        }
    });

    // println!("{:?}", sorted_by_rank);

    let result = sorted_by_rank
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| {
            let rank = u32::try_from(i + 1).unwrap();
            return *(bid) * rank;
        })
        .sum::<u32>();

    println!("Result: {}", result);
}

const FIVE_OF_A_KIND: u32 = 7;
const FOUR_OF_A_KIND: u32 = 6;
const FULL_HOUSE: u32 = 5;
const THREE_OF_A_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

fn map_hand_to_value_with_joker(x: &String) -> u32 {
    let mut chars = x.clone().chars().filter(|c| c != &'J').collect::<Vec<_>>();
    let jokers_count = x.clone().chars().filter(|c| c == &'J').count();
    chars.sort();
    chars.dedup();
    let count_of_figures = chars
        .iter()
        .map(|c| x.chars().filter(|x| x == c).count())
        .collect::<Vec<_>>();

    if jokers_count == 0 {
        if count_of_figures.contains(&5) {
            return FIVE_OF_A_KIND;
        } else if count_of_figures.contains(&4) {
            return FOUR_OF_A_KIND;
        } else if count_of_figures.contains(&3) && count_of_figures.contains(&2) {
            return FULL_HOUSE;
        } else if count_of_figures.contains(&3) {
            return THREE_OF_A_KIND;
        } else if count_of_figures.contains(&2) {
            if count_of_figures.iter().filter(|x| x == &&2).count() == 2 {
                return TWO_PAIR;
            }
            return ONE_PAIR;
        }
        return HIGH_CARD;
    }

    if jokers_count == 1 {
        if count_of_figures.contains(&4) {
            return FIVE_OF_A_KIND;
        } else if count_of_figures.contains(&3) {
            return FOUR_OF_A_KIND;
        } else if count_of_figures.contains(&2) {
            if count_of_figures.iter().filter(|x| x == &&2).count() == 2 {
                return FULL_HOUSE;
            }
            return THREE_OF_A_KIND;
        }
        return ONE_PAIR;
    }

    if jokers_count == 2 {
        if count_of_figures.contains(&3) {
            return FIVE_OF_A_KIND;
        } else if count_of_figures.contains(&2) {
            return FOUR_OF_A_KIND;
        }

        return THREE_OF_A_KIND;
    }

    if jokers_count == 3 {
        if count_of_figures.contains(&2) {
            return FIVE_OF_A_KIND;
        }
        return FOUR_OF_A_KIND;
    }

    if jokers_count == 4 || jokers_count == 5 {
        return FIVE_OF_A_KIND;
    }

    panic!();
}

fn map_char_to_value_with_joker(x: &char) -> u32 {
    const RADIX: u32 = 10;
    return match x.to_digit(RADIX) {
        Some(it) => it,
        _ => match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => panic!("{}", x),
        },
    };
}
