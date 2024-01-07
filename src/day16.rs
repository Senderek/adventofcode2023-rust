use std::fmt;

use crate::utils;

pub fn solve_16() {
    let mut map: [[char; 110]; 110] = [[('.'); 110]; 110];
    let mut memory: [[(bool, bool, bool, bool); 110]; 110] =
        [[(false, false, false, false); 110]; 110];
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input16_demo.txt").unwrap();

    let strings = lines
        .map(|line| {
            let line = line.unwrap();
            line
        })
        .collect::<Vec<_>>();

    for (i, ln) in strings.iter().enumerate() {
        for (j, c) in ln.chars().enumerate() {
            if c != '.' {
                map[j][i] = c;
            }
        }
    }

    // println!("{:?}", map);

    let nubmer_of_rows = strings.len();
    let number_of_columns = get_number_of_columns(strings);
    if number_of_columns != nubmer_of_rows {
        panic!();
    }
    let max = number_of_columns;

    println!(
        "Number of rows: {}. Number of columns: {}",
        nubmer_of_rows, number_of_columns
    );

    let starting_point: (usize, usize, Direction) = (0, 0, Direction::Right);
    let mut cur_beams = get_new_position(
        starting_point.0,
        starting_point.1,
        starting_point.2,
        &map,
        &mut memory,
        max,
    );
    // println!("{:?}", cur_beams);
    let mut turn = 0;
    while cur_beams.len() > 0 {
        let mut r_beams = vec![];
        for i in cur_beams.iter() {
            let mut new_beams = get_new_position(i.0, i.1, i.2, &map, &mut memory, max);
            r_beams.append(&mut new_beams);
        }
        cur_beams = r_beams
            .iter()
            .filter(|b| {
                let m = memory[b.0][b.1];
                let r = match b.2 {
                    Direction::Up => m.0,
                    Direction::Down => m.2,
                    Direction::Left => m.3,
                    Direction::Right => m.1,
                };
                return !r;
            })
            .map(|x| (*x))
            .collect::<Vec<(usize, usize, Direction)>>();
        println!("Turn: {:?}. Numbers of beams: {}", turn, cur_beams.len());
        // println!("{:?}", cur_beams);
        turn = turn + 1;
        // if turn > 20 {
        //     break;
        // }
    }
    print_energaized(memory, max);

    let result: usize = (0..nubmer_of_rows)
        .map(|y| {
            (0..number_of_columns)
                .filter(|x| {
                    let m = memory[*x][y];
                    return m.0 || m.1 || m.2 || m.3;
                })
                .count()
        })
        .sum();

    println!("Result: {}", result);
}

pub fn solve_16b() {
    let mut map: [[char; 110]; 110] = [[('.'); 110]; 110];

    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input16.txt").unwrap();

    let strings = lines
        .map(|line| {
            let line = line.unwrap();
            line
        })
        .collect::<Vec<_>>();

    for (i, ln) in strings.iter().enumerate() {
        for (j, c) in ln.chars().enumerate() {
            if c != '.' {
                map[j][i] = c;
            }
        }
    }

    // println!("{:?}", map);

    let nubmer_of_rows = strings.len();
    let number_of_columns = get_number_of_columns(strings);
    if number_of_columns != nubmer_of_rows {
        panic!();
    }
    let max = number_of_columns;

    println!(
        "Number of rows: {}. Number of columns: {}",
        nubmer_of_rows, number_of_columns
    );

    let starting_points = generate_starting_points(max);
    let result = starting_points
        .iter()
        .map(|x| {
            let mut memory: [[(bool, bool, bool, bool); 110]; 110] =
                [[(false, false, false, false); 110]; 110];
            let mut cur_beams = vec![*x];
            while cur_beams.len() > 0 {
                let mut r_beams = vec![];
                for i in cur_beams.iter() {
                    let mut new_beams = get_new_position(i.0, i.1, i.2, &map, &mut memory, max);
                    r_beams.append(&mut new_beams);
                }
                cur_beams = r_beams
                    .iter()
                    .filter(|b| {
                        let m = memory[b.0][b.1];
                        let r = match b.2 {
                            Direction::Up => m.0,
                            Direction::Down => m.2,
                            Direction::Left => m.3,
                            Direction::Right => m.1,
                        };
                        return !r;
                    })
                    .map(|x| (*x))
                    .collect::<Vec<(usize, usize, Direction)>>();
            }

            let result: usize = (0..nubmer_of_rows)
                .map(|y| {
                    (0..number_of_columns)
                        .filter(|x| {
                            let m = memory[*x][y];
                            return m.0 || m.1 || m.2 || m.3;
                        })
                        .count()
                })
                .sum();

            result
        })
        .max();

    println!("Result: {}", result.unwrap());
}

fn get_number_of_columns(v: Vec<String>) -> usize {
    v.first().unwrap().len()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "/\\"),
            Direction::Down => write!(f, "\\/"),
            Direction::Left => write!(f, "<-"),
            Direction::Right => write!(f, "->"),
        }
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "/\\"),
            Direction::Down => write!(f, "\\/"),
            Direction::Left => write!(f, "<-"),
            Direction::Right => write!(f, "->"),
        }
    }
}

const EMPTY: Vec<(usize, usize, Direction)> = vec![];

fn get_new_position(
    cur_x: usize,
    cur_y: usize,
    direction: Direction,
    map: &[[char; 110]; 110],
    memory: &mut [[(bool, bool, bool, bool); 110]; 110],
    max: usize,
) -> Vec<(usize, usize, Direction)> {
    let cur_elem = map[cur_x][cur_y];
    let cur_memory = memory[cur_x][cur_y];
    match direction {
        Direction::Down => {
            memory[cur_x][cur_y] = (cur_memory.0, cur_memory.1, true, cur_memory.3);
            match cur_elem {
                '.' | '|' => {
                    if max > cur_y + 1 {
                        return vec![(cur_x, cur_y + 1, Direction::Down)];
                    } else {
                        return EMPTY;
                    }
                }
                '/' => {
                    if cur_x == 0 {
                        return EMPTY;
                    } else {
                        return vec![(cur_x - 1, cur_y, Direction::Left)];
                    }
                }
                '\\' => {
                    if max > cur_x + 1 {
                        return vec![(cur_x + 1, cur_y, Direction::Right)];
                    } else {
                        return EMPTY;
                    }
                }
                '-' => {
                    let mut r: Vec<_> = vec![];
                    if cur_x != 0 {
                        r.push((cur_x - 1, cur_y, Direction::Left));
                    }
                    if max > cur_x + 1 {
                        r.push((cur_x + 1, cur_y, Direction::Right));
                    }
                    return r;
                }
                _ => panic!(),
            }
        }
        Direction::Up => {
            memory[cur_x][cur_y] = (true, cur_memory.1, cur_memory.2, cur_memory.3);
            match cur_elem {
                '.' | '|' => {
                    if cur_y != 0 {
                        return vec![(cur_x, cur_y - 1, Direction::Up)];
                    } else {
                        return EMPTY;
                    }
                }
                '/' => {
                    if max > cur_x + 1 {
                        return vec![(cur_x + 1, cur_y, Direction::Right)];
                    } else {
                        return EMPTY;
                    }
                }
                '\\' => {
                    if cur_x == 0 {
                        return EMPTY;
                    } else {
                        return vec![(cur_x - 1, cur_y, Direction::Left)];
                    }
                }
                '-' => {
                    let mut r: Vec<_> = vec![];
                    if cur_x != 0 {
                        r.push((cur_x - 1, cur_y, Direction::Left));
                    }
                    if max > cur_x + 1 {
                        r.push((cur_x + 1, cur_y, Direction::Right));
                    }
                    return r;
                }
                _ => panic!(),
            }
        }
        Direction::Left => {
            memory[cur_x][cur_y] = (cur_memory.0, cur_memory.1, cur_memory.2, true);
            match cur_elem {
                '.' | '-' => {
                    if cur_x != 0 {
                        return vec![(cur_x - 1, cur_y, Direction::Left)];
                    } else {
                        return EMPTY;
                    }
                }
                '/' => {
                    if max > cur_y + 1 {
                        return vec![(cur_x, cur_y + 1, Direction::Down)];
                    } else {
                        return EMPTY;
                    }
                }
                '\\' => {
                    if cur_y == 0 {
                        return EMPTY;
                    } else {
                        return vec![(cur_x, cur_y - 1, Direction::Up)];
                    }
                }
                '|' => {
                    let mut r: Vec<_> = vec![];
                    if cur_y != 0 {
                        r.push((cur_x, cur_y - 1, Direction::Up));
                    }
                    if max > cur_y + 1 {
                        r.push((cur_x, cur_y + 1, Direction::Down));
                    }
                    return r;
                }
                _ => panic!(),
            }
        }
        Direction::Right => {
            memory[cur_x][cur_y] = (cur_memory.0, true, cur_memory.2, cur_memory.3);
            match cur_elem {
                '.' | '-' => {
                    if max > (cur_x + 1) {
                        return vec![(cur_x + 1, cur_y, Direction::Right)];
                    } else {
                        return EMPTY;
                    }
                }
                '/' => {
                    if cur_y == 0 {
                        return EMPTY;
                    } else {
                        return vec![(cur_x, cur_y - 1, Direction::Up)];
                    }
                }
                '\\' => {
                    if max > cur_y + 1 {
                        return vec![(cur_x, cur_y + 1, Direction::Down)];
                    } else {
                        return EMPTY;
                    }
                }
                '|' => {
                    let mut r: Vec<_> = vec![];
                    if cur_y != 0 {
                        r.push((cur_x, cur_y - 1, Direction::Up));
                    }
                    if max > cur_y + 1 {
                        r.push((cur_x, cur_y + 1, Direction::Down));
                    }
                    return r;
                }
                _ => panic!(),
            }
        }
    }
}

fn print_energaized(memory: [[(bool, bool, bool, bool); 110]; 110], max: usize) {
    for i in 0..max {
        for j in 0..max {
            let m = memory[j][i];
            print!("{}", if m.0 || m.1 || m.2 || m.3 { '#' } else { '.' });
        }
        println!("");
    }
}

fn generate_starting_points(max: usize) -> Vec<(usize, usize, Direction)> {
    let mut v1 = (0..max)
        .map(|x| (0, x, Direction::Right))
        .collect::<Vec<_>>();
    let v2 = (0..max)
        .map(|x| (x, 0, Direction::Down))
        .collect::<Vec<_>>();
    let v3 = (0..max)
        .map(|x| (max - 1, x, Direction::Left))
        .collect::<Vec<_>>();
    let v4 = (0..max)
        .map(|x| (x, max - 1, Direction::Up))
        .collect::<Vec<_>>();
    v1.extend(v2);
    v1.extend(v3);
    v1.extend(v4);
    v1
}
