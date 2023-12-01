use onig::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RADIX: u32 = 10;

fn main() {
    // File must exist in the current path
    if let Ok(lines) = read_lines("C:/rep/adventofcode2023/src/inputs/input1_test.txt") {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(ip: &String) -> u32 {
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
    let regex = Regex::new(r"(?iU)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)")
        .unwrap();

    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;

    for (_, [dgt]) in regex.captures_iter(ip).map(|c| c.extract()) {
        match dgt.parse::<u32>() {
            Ok(n) => match first_digit {
                Some(_) => second_digit = Some(n),
                None => first_digit = Some(n),
            },
            _ => {
                match dgt {
                    "one" => match first_digit  { Some(_) => second_digit = Some(1), None => first_digit = Some(1) },
                    "two" => match first_digit  { Some(_) => second_digit =  Some(2), None => first_digit =  Some(2) },
                    "three" => match first_digit  { Some(_) => second_digit =  Some(3), None => first_digit =  Some(3) },
                    "four" => match first_digit  { Some(_) => second_digit =  Some(4), None => first_digit =  Some(4) },
                    "five" => match first_digit  { Some(_) => second_digit =  Some(5), None => first_digit =  Some(5) },
                    "six" => match first_digit  { Some(_) => second_digit =  Some(6), None => first_digit =  Some(6) },
                    "seven" => match first_digit  { Some(_) => second_digit =  Some(7), None => first_digit =  Some(7) },
                    "eight" => match first_digit  { Some(_) => second_digit =  Some(8), None => first_digit =  Some(8) },
                    "nine" => match first_digit  { Some(_) => second_digit =  Some(9), None => first_digit =  Some(9) },
                    _ => panic!("oh no!"),
                }
            },
        }
    }

    return match (first_digit, second_digit) {
        (Some(f), Some(s)) => f * 10 + s,
        (Some(f), None) => f * 10 + f,
        _ => panic!("oh zero!"),
    };
}
