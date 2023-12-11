use crate::utils;

#[allow(dead_code)]
pub fn solve_6a() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input6.txt").unwrap();
    let mut i: usize = 0;
    let mut times: Vec<u32> = [].to_vec();
    let mut distances: Vec<u32> = [].to_vec();

    for l in lines {
        if let Ok(line) = l {
            if i == 0 {
                times = line
                    .split(':')
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
            }
            if i == 1 {
                distances = line
                    .split(':')
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
            }
        }
        i = i + 1;
    }
    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);

    let pairs: Vec<_> = times
        .into_iter()
        .enumerate()
        .map(|(index, t)| (t, *(distances.get(index).unwrap())))
        .collect();
    println!("Pairs: {:?}", pairs);
    let result = pairs
        .iter()
        .map(|(time, distance)| {
            let mut records = 0;
            for speed in 1..*time {
                let time_to_travel = time - speed;
                let travel_distance = time_to_travel * speed;
                println!(
                    "Hold the button for {}, {} to move, {} total",
                    speed, time_to_travel, travel_distance
                );
                if &travel_distance > distance {
                    records = records + 1;
                }
            }
            return records;
        })
        .filter(|r| *r > 0)
        .fold(1, |acc, r| acc * r);
    println!("Result: {}", result);
}

#[allow(dead_code)]
pub fn solve_6b() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input6.txt").unwrap();
    let mut i: usize = 0;
    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for l in lines {
        if let Ok(line) = l {
            if i == 0 {
                time = line
                    .split(':')
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()
                    .join("")
                    .parse::<u64>()
                    .unwrap()
            }
            if i == 1 {
                distance = line
                    .split(':')
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()
                    .join("")
                    .parse::<u64>()
                    .unwrap()
            }
        }
        i = i + 1;
    }
    println!("Time: {:?}", time);
    println!("Distance: {:?}", distance);

    let range = 1..time;
    let records = range
        .filter(|speed| {
            let time_to_travel = time - speed;
            let travel_distance = time_to_travel * speed;

            if travel_distance > distance {
                return true;
            }
            return false;
        })
        .count();

    println!("Result: {}", records);
}
