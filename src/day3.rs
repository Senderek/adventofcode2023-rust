use regex::Regex;

use crate::utils;

#[allow(dead_code)]
pub fn solve_3a() {
    let regex = Regex::new(r"([0-9]+)").unwrap();
    let mut results: Vec<(u32, usize, usize, usize)> = vec![];

    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input3.txt") {
        let result: Vec<String> = lines
            .into_iter()
            .enumerate()
            .map(|(xind, l)| {
                if let Ok(ip) = l {
                    for m in regex.find_iter(&ip) {
                        let string_start = m.start();
                        let string_end = m.end();
                        let nmb = m.as_str().parse::<u32>().unwrap();
                        results.push((nmb, xind, string_start, string_end));
                    }
                    return ip;
                }
                panic!("Cannot parse line {}!", xind);
            })
            .collect();

        let sum = results
            .into_iter()
            .map(|(nmb, xind, string_start, string_end)| {
                if xind > 0 {
                    let l = result.get(xind - 1).unwrap();

                    if includes_symbols(l, string_start, string_end) {
                        return nmb;
                    }
                }
                if includes_symbols(result.get(xind).unwrap(), string_start, string_end) {
                    return nmb;
                }
                if xind + 1 < result.len() {
                    let l = result.get(xind + 1).unwrap();

                    if includes_symbols(l, string_start, string_end) {
                        return nmb;
                    }
                }
                0
            })
            .sum::<u32>();
        println!("{}", sum);
    } else {
        println!("File not OK");
    }
}

#[allow(dead_code)]
pub fn solve_3b() {
    println!("_____________________________________________");
    let regex = Regex::new(r"([0-9]+)").unwrap();
    let mut results: Vec<(u32, usize, usize, usize)> = vec![];
    let mut pot_gears = vec![];

    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input3.txt") {
        let _: Vec<_> = lines
            .into_iter()
            .enumerate()
            .map(|(xind, l)| {
                if let Ok(ip) = l {
                    for m in regex.find_iter(&ip) {
                        let string_start = m.start();
                        let string_end = m.end();
                        let nmb = m.as_str().parse::<u32>().unwrap();
                        results.push((nmb, xind, string_start, string_end));
                    }

                    for m in ip.match_indices('*') {
                        pot_gears.push((xind, m.0));
                    }
                    return ip;
                }
                panic!("Cannot parse line {}!", xind);
            })
            .collect();

        let mut sum = 0;
        for (pg_row, pg_column) in pot_gears {
            // println!("@");
            // println!("potential gear: {}:{}", pg_row, pg_column);
            let mut i = 0;
            let mut pwr = 1;
            for (nmb, nmb_row, nmb_column_start, nmb_column_end) in results.iter() {
                if nmb_row >= &(pg_row - 1) && nmb_row <= &(pg_row + 1) {
                    // println!(
                    //     "nmb: {nmb}, row: {}, s: {}, e: {}",
                    //     nmb_row, nmb_column_start, nmb_column_end
                    // );
                    // println!(
                    //     "{} <= {} == {} && {} >= {} = {}",
                    //     &pg_column,
                    //     nmb_column_end,
                    //     &pg_column <= nmb_column_end,
                    //     &pg_column,
                    //     nmb_column_start,
                    //     &pg_column >= nmb_column_start
                    // );
                    if &pg_column <= nmb_column_end && &(pg_column + 1) >= nmb_column_start {
                        // println!("adjencent number: {} {}:{}", nmb, nmb_column_start, nmb_column_end);
                        i = i + 1;
                        pwr = pwr * nmb;
                    }
                }
                if i > 2 {
                    break;
                }
            }

            if i == 2 {
                println!("power: {}. {}:{}", pwr, pg_row, pg_column);
                sum = sum + pwr;
            }
        }
        println!("{}", sum)
    }
}

fn includes_symbols(l: &str, string_start: usize, string_end: usize) -> bool {
    let s = if string_start > 0 {
        string_start - 1
    } else {
        0
    };
    let e: usize = if string_end + 1 <= l.len() {
        string_end + 1
    } else {
        l.len()
    };
    let substr = &l[s..e];
    let r = includes_a_symbol(substr);
    return r;
}

fn includes_a_symbol(s: &str) -> bool {
    s.chars().any(|x| {
        if x == '.'
            || x == '0'
            || x == '1'
            || x == '2'
            || x == '3'
            || x == '4'
            || x == '5'
            || x == '6'
            || x == '7'
            || x == '8'
            || x == '9'
        {
            return false;
        }
        return true;
    })
}
