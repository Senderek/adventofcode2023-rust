use crate::utils;

pub fn solve_10a() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input10_demo.txt").unwrap();
    let mut s_i: usize = 0;
    let mut s_j: usize = 0;
    let mut max_j: usize = 0;
    let map: Vec<_> = lines
        .enumerate()
        .map(|(i_usize, x)| {
            let line = x.unwrap();
            if line.contains("S") {
                let j_usize = line.find('S').unwrap();
                s_i = i_usize;
                s_j = j_usize;
                max_j = line.len();
            }
            return line.chars().collect::<Vec<_>>();
        })
        .collect();

    let max_i: usize = map.len();

    // println!("S: {} {}.", s_i, s_j);
    // println!("{:?}.", map);

    let i_range = get_edge(s_i, max_i);
    let j_range = get_edge(s_j, max_j);

    // println!("{:?} {:?}", i_range, j_range);

    let mut hits = 0;

    for i in i_range.0..=i_range.1 {
        for j in j_range.0..=j_range.1 {
            if i == s_i && j == s_j {
                continue;
            }

            let next = get_next(&map, i, j);
            if let Ok((next1, next2)) = next {
                // println!("{} {}: {:?} {:?} {:?}", i, j, next1, next2, (s_i, s_j));
                if next1.0 == s_i && next1.1 == s_j {
                    hits = hits + 1;
                    println!("Hit: {} {}", next2.0, next2.1);
                    let steps = get_next_with_exclusion(&map, i, j, &(s_i, s_j), 2, &(s_i, s_j));
                    // let steps = get_next_with_exclusion(
                    //     &map,
                    //     next2.0,
                    //     next2.1,
                    //     (next1.0, next1.1),
                    //     0,
                    //     (s_i, s_j),
                    // );
                    println!("Steps for full circle: {}", steps);
                    break;
                }

                if next2.0 == s_i && next2.1 == s_j {
                    hits = hits + 1;
                    let steps = get_next_with_exclusion(&map, i, j, &(s_i, s_j), 2, &(s_i, s_j));
                    println!("Steps for full circle: {}", steps);
                    break;
                }
            }
        }
    }
    println!("Hits: {}", hits);
}

fn get_edge(string_start: usize, max: usize) -> (usize, usize) {
    let s = if string_start > 0 {
        string_start - 1
    } else {
        0
    };
    let e: usize = if string_start + 1 <= max {
        string_start + 1
    } else {
        max
    };

    return (s, e);
}

fn get_next(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> Result<((usize, usize), (usize, usize)), &'static str> {
    let i_usize = usize::try_from(i).unwrap();
    let j_usize = usize::try_from(j).unwrap();
    let symbol_line = map.get(i_usize);
    if let Some(line) = symbol_line {
        let symbol = line.get(j_usize);
        if let Some(result) = symbol {
            return match result {
                '|' => Ok(((i - 1, j), (i + 1, j))),
                '-' => Ok(((i, j - 1), (i, j + 1))),
                'L' => Ok(((i - 1, j), (i, j + 1))),
                'J' => Ok(((i - 1, j), (i, j - 1))),
                '7' => Ok(((i, j - 1), (i + 1, j))),
                'F' => Ok(((i, j + 1), (i + 1, j))),
                _ => {
                    // println!("{} {}: Invalid symbol {}", i, j, result);
                    return Err("Invalid symbol");
                }
            };
        }
    }
    // println!("{} {}: Out of range", i, j);
    return Err("Out of range");
}

fn get_next_with_exclusion(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    prev: &(usize, usize),
    step: usize,
    start: &(usize, usize),
) -> usize {
    let g = get_next(&map, i, j).unwrap();
    let n = g.0 .0 == prev.0 && g.0 .1 == prev.1;
    let (next_i, next_j) = if n { g.1 } else { g.0 };
    // if step < 100 {
    println!("Step: {}. {} {}", step, next_i, next_j);
    if step == 1511 {
        println!("Overflow: {}. {} {}", step, next_i, next_j);
    }
    // }
    if next_i == start.0 && next_j == start.1 {
        return step;
    } else {
        return get_next_with_exclusion(map, next_i, next_j, &(i, j), step + 1, start);
    }
}

fn get_next_with_replacement(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    prev: &(usize, usize),
    step: usize,
    start: &(usize, usize),
) -> usize {
    let g = get_next(&map, i, j).unwrap();
    let n = g.0 .0 == prev.0 && g.0 .1 == prev.1;
    let (next_i, next_j) = if n { g.1 } else { g.0 };
    println!("Step: {}. {} {}", step, next_i, next_j);
    if next_i == start.0 && next_j == start.1 {
        return step;
    } else {
        return get_next_with_exclusion(map, next_i, next_j, &(i, j), step + 1, start);
    }
}
