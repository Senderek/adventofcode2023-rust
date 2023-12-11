use crate::utils;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve_8a() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input8.txt").unwrap();

    let mut lr_instructions: Vec<char> = [].to_vec();
    let mut map = HashMap::new();

    let mut i: usize = 0;
    for l in lines {
        let line = l.unwrap();
        if i == 0 {
            lr_instructions = line.trim().chars().collect::<Vec<_>>();
        }

        if i > 1 {
            let (id_part, instructions_part) = line.split_once('=').unwrap();
            let id = id_part.trim().to_string();
            let (l_part, r_part) = instructions_part.split_once(',').unwrap();
            let l_cord = l_part.replace('(', "").trim().to_string();
            let r_cord = r_part.replace(')', "").trim().to_string();
            map.insert(id, (l_cord, r_cord));
        }

        i = i + 1;
    }

    println!("{:?}", lr_instructions);
    println!("{:?}", map);

    const FINAL_DESTINATION: &str = "ZZZ";
    let mut current_destination = "AAA";

    let mut step: usize = 0;
    while current_destination != FINAL_DESTINATION {
        let current_instruction = lr_instructions.get(step % lr_instructions.len()).unwrap();
        let destination_map = map.get_key_value(current_destination).unwrap();
        let new_destination = match current_instruction {
            'L' => &destination_map.1 .0,
            'R' => &destination_map.1 .1,
            _ => panic!("No instructions for: {}", current_instruction),
        };

        if new_destination == current_destination {
            panic!("Loop")
        } else {
            current_destination = new_destination;
        }

        step = step + 1;
    }

    println!("Result: {}", step);
}

#[allow(dead_code)]
pub fn solve_8b() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input8.txt").unwrap();

    let mut lr_instructions: Vec<char> = [].to_vec();
    let mut map = HashMap::new();
    let mut positions: Vec<_> = [].to_vec();

    let mut i: usize = 0;
    for l in lines {
        let line = l.unwrap();
        if i == 0 {
            lr_instructions = line.trim().chars().collect::<Vec<_>>();
        }

        if i > 1 {
            let (id_part, instructions_part) = line.split_once('=').unwrap();
            let id = id_part.trim().to_string();
            let id2 = id.clone();
            if id2.clone().ends_with('A') == true {
                positions.push(id2);
            }
            let (l_part, r_part) = instructions_part.split_once(',').unwrap();
            let l_cord = l_part.replace('(', "").trim().to_string();
            let r_cord = r_part.replace(')', "").trim().to_string();
            map.insert(id, (l_cord, r_cord));
        }

        i = i + 1;
    }

    println!("{:?}", positions);

    let minimal: Vec<_> = positions
        .iter()
        .map(|x| {
            let mut current_destination = x.clone();
            let mut step: u64 = 0;
            while !current_destination.ends_with("Z") {
                let current_instruction = lr_instructions
                    .get(usize::try_from(step).unwrap() % lr_instructions.len())
                    .unwrap();
                let destination_map = map.get_key_value(&current_destination).unwrap();
                let new_destination = match current_instruction {
                    'L' => &destination_map.1 .0,
                    'R' => &destination_map.1 .1,
                    _ => panic!("No instructions for: {}", current_instruction),
                };

                if new_destination == &current_destination {
                    panic!("Loop")
                } else {
                    current_destination = new_destination.clone();
                }

                step = step + 1;
            }
            step
        })
        .collect();

    println!("{:?}", minimal);

    let mut result = *minimal.get(0).unwrap();
    for i in minimal 
    {
        if i == 0 
        {
            continue;
        }
        result = lcm(&result, &i);
    }

    println!("Result: {}", result);
}

fn gcd(a: &u64, b: &u64) -> u64 {
    let mut b = b.clone();
    let mut a = a.clone();
    // Euclidean algorithm
    while b != 0 {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a: &u64, b: &u64) -> u64 {
    return a * b / gcd(a, b);
}
