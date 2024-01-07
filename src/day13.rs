use crate::utils;

pub fn solve_13() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input13.txt").unwrap();

    let mut buf: Vec<String> = vec![];
    let mut maps: Vec<Vec<String>> = vec![];

    lines.for_each(|line| {
        let line = line.unwrap();
        // println!("{}", line);
        if line.is_empty() {
            maps.push(buf.clone());
            buf.clear();
        } else {
            buf.push(line.clone());
        }
    });

    if !buf.is_empty() {
        maps.push(buf);
    }

    let vertical_score: usize = maps.iter().map(|x| map_to_points(x)).sum();
    println!("");
    let horizontal_maps: Vec<Vec<String>> = maps
        .iter()
        .map(|m| {
            let c: Vec<Vec<char>> = m.iter().map(|y| y.chars().into_iter().collect()).collect();
            let fliped = flip_left(c);
            println!("{:?}", fliped);
            fliped
                .iter()
                .map(|f| {
                    let r = f.into_iter().collect::<String>();
                    r
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let horizontal_score: usize = horizontal_maps.iter().map(|x| map_to_points(x) * 100).sum();

    println!(
        "Veritcal score: {}. Horizontal score: {}. Result: {}",
        vertical_score,
        horizontal_score,
        vertical_score + horizontal_score
    );
}

fn map_to_points(x: &Vec<String>) -> usize {
    let columns = x[0].len();
    (1..columns)
        .map(|c| {
            if x.iter().all(|lin| {
                let sl = if c < columns - c {
                    lin[..(c * 2)].chars().collect::<Vec<char>>()
                } else {
                    // println!("{} {} {} {}", lin, c, columns - c, columns);
                    let rev_c = columns - c;
                    let g = columns - (rev_c * 2);
                    lin[g..].chars().collect::<Vec<char>>()
                };
                // println!(
                //     "Slice ({}): {:?}",
                //     c,
                //     sl.clone().into_iter().collect::<String>()
                // );
                return is_palindrome(&sl);
            }) {
                return c.clone();
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn is_palindrome(input: &Vec<char>) -> bool {
    let mut i = 0;

    while i < (input.len() / 2) {
        if input[i] != input[input.len() - 1 - i] {
            return false;
        }

        i += 1;
    }

    true
}

fn flip_left(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let reversed: Vec<Vec<char>> = v
        .iter()
        .map(|x| x.iter().rev().map(|x| *x).collect())
        .collect();
    transpose(reversed)
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
