use crate::utils;
use std::ops::RangeInclusive;

#[allow(dead_code)]
pub fn solve_5a() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input5.txt").unwrap();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut seeds: Vec<u64> = [].to_vec();
    let mut current_maps: Vec<(u64, u64, u64)> = [].to_vec();

    for l in lines {
        if let Ok(line) = l {
            if i == 0 {
                seeds = line
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                println!("Seeds: {:?}", seeds);
                i = i + 1;
                continue;
            }
            if i == 1 || i == 2 {
                i = i + 1;
                continue;
            }

            if line.trim().is_empty() {
                seeds = seeds
                    .into_iter()
                    .map(|x| {
                        for &(destination_range_start, source_range_start, range_length) in
                            current_maps.iter()
                        {
                            if range_length == 0 {
                                continue;
                            }

                            if x >= source_range_start && x < source_range_start + range_length {
                                println!(
                                    "{} + {} - {} from {:?}",
                                    destination_range_start,
                                    source_range_start,
                                    x,
                                    (destination_range_start, source_range_start, range_length)
                                );
                                return destination_range_start + (x - source_range_start);
                            }
                        }
                        x
                    })
                    .collect();
                println!("{} result: {:?}", j, seeds);
                current_maps = [].to_vec();
                j = j + 1;
                println!(" ");
                i = i + 1;
                continue;
            }

            if line.contains(":") {
                i = i + 1;
                continue;
            }

            let dsr = line
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            current_maps.push((
                *dsr.get(0).unwrap(),
                *dsr.get(1).unwrap(),
                *dsr.get(2).unwrap(),
            ))
        }
        i = i + 1;
    }

    println!("Locations: {:?}", seeds);
    println!("Result: {}", seeds.iter().min().unwrap());
}

pub fn solve_5b() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input5_demo.txt").unwrap();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut seeds: Vec<_> = [].to_vec();
    let mut seeds2: Vec<_> = [].to_vec();
    let mut current_maps: Vec<(RangeInclusive<u64>, u64, bool)> = [].to_vec();

    for l in lines {
        if let Ok(line) = l {
            if i == 0 {
                seeds = line
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
                    .chunks(2)
                    .filter(|y| y[1] > 0)
                    .map(|x| (x[0]..=x[0] + x[1] - 1))
                    .collect();
                seeds2 = line
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
                    .chunks(2)
                    .filter(|y| y[1] > 0)
                    .map(|x| (x[0], x[0] + x[1] - 1))
                    .collect();
                println!("Seeds: {:?}", seeds);
                println!("");
                i = i + 1;
                continue;
            }
            if i == 1 || i == 2 {
                i = i + 1;
                continue;
            }

            if line.trim().is_empty() {
                println!("Ranges {:?}", current_maps);

                current_maps = [].to_vec();
                j = j + 1;
                i = i + 1;
                continue;
            }

            if line.contains(":") {
                i = i + 1;
                continue;
            }

            let dsr = line
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let destination_range_start = *dsr.get(0).unwrap();
            let source_range_start = *dsr.get(1).unwrap();
            let range_length: u64 = *dsr.get(2).unwrap();

            if range_length > 0 {
                let source_range = (source_range_start, source_range_start + range_length - 1);
                let destination_range =
                    destination_range_start..=destination_range_start + range_length - 1;
                let mut r: u64;
                let mut p: bool;
                if destination_range_start > source_range_start {
                    r = destination_range_start - source_range_start;
                    p = true;
                } else {
                    r = destination_range_start - source_range_start;
                    p = false;
                }
                current_maps.push((source_range, r, p));
            }
        }
        i = i + 1;
    }

    // println!("Result: {}", seeds.iter().min().unwrap());
}

// fn range_overlap(r1: RangeInclusive<u64>, r2: RangeInclusive<u64>) -> bool {
//     let x1 = r1.min().unwrap();
//     let x2 = r1.max().unwrap();
//     let y1 = r2.min().unwrap();
//     let y2 = r2.max().unwrap();
//     return max(x1, y1) <= min(x2, y2);
// }

// fn max(x1: u64, x2: u64) -> u64 {
//     if x1 > x2 {
//         return x1;
//     }
//     return x2;
// }

// fn min(x1: u64, x2: u64) -> u64 {
//     if x1 < x2 {
//         return x1;
//     }
//     return x2;
// }
