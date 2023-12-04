use regex::Regex;

use crate::utils;

#[allow(dead_code)]
pub fn solve_1a() {
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input1.txt") {
        let result = lines
            .map(|l| {
                if let Ok(ip) = l {
                    let m = get_calibration_value(&ip);
                    println!("{} result {}", ip, m);
                    return m;
                }
                0
            })
            .sum::<u32>();
        println!("{} result", result);
    } else {
        println!("File not OK");
    }
}

#[allow(dead_code)]
pub fn solve_1b() {
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input1.txt") {
        let result = lines
            .map(|l| {
                if let Ok(ip) = l {
                    let m = get_calibartion_value_regex(&ip);
                    println!("{} result {}", ip, m);
                    return m;
                }
                0
            })
            .sum::<u32>();
        println!("{} result", result);
    } else {
        println!("File not OK");
    }
}

fn get_calibration_value(ip: &String) -> u32 {
    const RADIX: u32 = 10;

    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;
    for c in ip.chars() {
        match c.to_digit(RADIX) {
            Some(n) => match first_digit {
                Some(_) => second_digit = Some(n),
                None => first_digit = Some(n),
            },
            _ => (),
        }
    }
    return match (first_digit, second_digit) {
        (Some(f), Some(s)) => f * 10 + s,
        (Some(f), None) => f * 10 + f,
        _ => 0,
    };
}

fn get_calibartion_value_regex(ip: &String) -> u32 {
    let regex =
        Regex::new(r"(?iU)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)")
            .unwrap();

    let capt = regex.captures(ip);
    let second_digit = match capt {
        Some(e) => {
            let c = e.get(0).unwrap();
            let current_digit = string_to_number(c.as_str());
            let str = c.start();
            cpt(str, &regex, ip, current_digit)
        }
        _ => panic!("no first match"),
    };

    let first_digit = match regex.find(ip) {
        Some(s) => string_to_number(s.as_str()),
        _ => {
            println!("{}", ip);
            panic!("passed bullshit {}!", ip)
        }
    };

    return first_digit * 10 + second_digit;
}

fn cpt(start: usize, regex: &Regex, ip: &str, last_digit: u32) -> u32 {
    let slice = &ip[start + 1..];
    return match regex.captures(slice) {
        Some(e) => {
            let c = e.get(0).unwrap();
            let str = c.start();
            let current_digit = string_to_number(c.as_str());
            cpt(str, regex, slice, current_digit)
        }
        _ => last_digit,
    };
}

fn string_to_number(dgt: &str) -> u32 {
    match dgt.parse::<u32>() {
        Ok(n) => return n,
        _ => match dgt {
            "one" => return 1,
            "two" => return 2,
            "three" => return 3,
            "four" => return 4,
            "five" => return 5,
            "six" => return 6,
            "seven" => return 7,
            "eight" => return 8,
            "nine" => return 9,
            _ => panic!("passed bullshit {}!", dgt),
        },
    }
}
