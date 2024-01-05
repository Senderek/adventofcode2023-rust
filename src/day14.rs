use itertools::Itertools;
use std::cmp::Ordering;

use crate::utils;

#[allow(dead_code)]
pub fn solve_14_bad() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input14.txt").unwrap();

    let map: Vec<_> = lines
        .map(|line| {
            let line = line.unwrap();
            return line;
        })
        .collect();

    let mut cache: [usize; 100] = [0; 100];
    let mut cache2 = [false; 100];
    let columns = map.get(0).unwrap().chars().count();
    let rows = map.len();

    for x in map.iter() {
        for (j, y) in x.chars().enumerate() {
            if cache2[j] {
                continue;
            }

            if y == 'O' {
                cache[j] = cache[j] + 1;
            }

            if y == '#' {
                cache2[j] = true;
            }
        }
    }

    println!("");
    let result: usize = (0..columns)
        .map(|i| {
            let number_of_boulders = cache[i];
            let sum: usize = (0..number_of_boulders).map(|x| rows - x).sum();
            println!("column: {} = b: {} = sum: {}", i, number_of_boulders, sum);
            sum
        })
        .sum();

    println!("");
    println!("Result: {}", result);
}

#[allow(dead_code)]
pub fn solve_14() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input14.txt").unwrap();

    let map: Vec<Vec<char>> = lines
        .map(|line| {
            let line = line.unwrap();
            return line.chars().rev().collect();
        })
        .collect();

    let columns = map.len();

    // println!("{:?}", map);

    let tranposed = transpose(map);

    // pretty_print(tranposed);

    let result: usize = tranposed
        .iter()
        .map(|x| {
            let mut current_beam_position: usize = usize::MAX;
            let mut boulder_count: usize = 0;
            let mut total_sum: usize = 0;
            for (column, character) in x.iter().enumerate() {
                if *character == 'O' {
                    boulder_count = boulder_count + 1;
                }

                if *character == '#' {
                    let sum = sum_load(current_beam_position, columns, boulder_count);
                    // println!("Section sum: {}", sum);
                    total_sum = sum + total_sum;
                    current_beam_position = column;
                    boulder_count = 0;
                }
            }

            if boulder_count > 0 {
                let sum = sum_load(current_beam_position, columns, boulder_count);
                // println!("Section sum: {}", sum);
                total_sum = sum + total_sum;
            }

            println!("{:?}: {}", x, total_sum);
            total_sum
        })
        .sum();

    println!("Result: {}", result);
}

pub fn solve_14b() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input14.txt").unwrap();

    let original: Vec<Vec<char>> = lines
        .map(|line| {
            let line = line.unwrap();
            return line.chars().collect();
        })
        .collect();

    let columns = original.len();

    let north = roll_to_left(flip_left(original));

    let west = roll_to_left(flip_right(north));
    let south = roll_to_left(flip_right(west));
    let east = roll_to_left(flip_right(south));

    let res_east = (0..1000-1).fold(east, |x, i| {
        let n = roll_to_left(flip_right(x));
        let w = roll_to_left(flip_right(n));
        let s = roll_to_left(flip_right(w));
        if i % 10000 == 0 {
            println!("{} {}", i, i * 100 / (1000-1));
        }
        roll_to_left(flip_right(s))
    });

    println!("Total: {}", count_total_load2(flip_right(flip_right(res_east))));

    // for i in 0..1000000000 {
    //     let north = roll_to_left(flip_right(east.clone()));
    //     let west = roll_to_left(flip_right(north));
    //     let south = roll_to_left(flip_right(west));
    //     east = roll_to_left(flip_right(south));
    //     if i % 10000 == 0 {
    //         println!("{} {}", i, i * 100 / 1000000000);
    //     }
    // }

    // println!("Total: {}", count_total_load(roll_to_left(flip_right(res_east)), columns));
}

fn flip_left(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let reversed: Vec<Vec<char>> = v
        .iter()
        .map(|x| x.iter().rev().map(|x| *x).collect())
        .collect();
    transpose(reversed)
}

fn flip_right(mut v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    v.reverse();
    transpose(v)
}

fn roll_to_left(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    v.iter()
        .map(|row| {
            let s: String = row.into_iter().collect();
            let sorted = s
                .split('#')
                // .filter(|x| !x.is_empty())
                .map(|sl| {
                    sl.chars()
                        .sorted_by(|a, b| {
                            if a == &'O' && b != &'O' {
                                return Ordering::Less;
                            }

                            if b == &'O' && a != &'O' {
                                return Ordering::Greater;
                            }

                            Ordering::Equal
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>();

            sorted.join("#").chars().collect::<Vec<char>>()
        })
        .collect()
}

fn count_total_load(v: Vec<Vec<char>>, columns: usize) -> usize {
    v.iter()
        .map(|x| {
            let mut current_beam_position: usize = usize::MAX;
            let mut boulder_count: usize = 0;
            let mut total_sum: usize = 0;
            for (column, character) in x.iter().enumerate() {
                if *character == 'O' {
                    boulder_count = boulder_count + 1;
                }

                if *character == '#' {
                    let sum = sum_load(current_beam_position, columns, boulder_count);
                    // println!("Section sum: {}", sum);
                    total_sum = sum + total_sum;
                    current_beam_position = column;
                    boulder_count = 0;
                }
            }

            if boulder_count > 0 {
                let sum = sum_load(current_beam_position, columns, boulder_count);
                // println!("Section sum: {}", sum);
                total_sum = sum + total_sum;
            }

            println!("{:?}: {}", x, total_sum);
            total_sum
        })
        .sum()
}

fn count_total_load2(v: Vec<Vec<char>>) -> usize {
    let number_of_rows = v.len();
    v.iter()
        .enumerate()
        .map(|(g, x)| {
            let mut total_sum: usize = 0;
            for character in x.iter() {
                if *character == 'O' {
                    total_sum = number_of_rows - g + total_sum;
                }
            }

            println!("{:?}: {}", x, number_of_rows - g);
            total_sum
        })
        .sum()
}

fn pretty_print(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    v.iter().map(|x| println!("{:?}", x)).for_each(drop);
    v
}

fn sum_load(current_beam_position: usize, total_columns: usize, boulder_count: usize) -> usize {
    let starting_value: usize = if current_beam_position == usize::MAX {
        total_columns
    } else {
        total_columns - current_beam_position - 1
    };
    let sum: usize = (0..boulder_count).map(|x| starting_value - x).sum();
    sum
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
