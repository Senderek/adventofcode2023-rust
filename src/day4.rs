use crate::utils;

#[allow(dead_code)]
pub fn solve_4a() {
    const TWO: usize = 2;
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input4.txt") {
        let result = lines
            .map(|l| {
                if let Ok(ip) = l {
                    let parts: Vec<&str> = ip.split([':', '|']).collect();
                    let player_numbers = parts
                        .get(2)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<u32>().unwrap());
                    let winning_numbers: Vec<u32> = parts
                        .get(1)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();

                    let winners = player_numbers.filter(|x| winning_numbers.contains(x));
                    let winners_collected: Vec<u32> = winners.collect();
                    let pow: u32 = u32::try_from(winners_collected.len()).unwrap();
                    return if pow == 0 { 0 } else { TWO.pow(pow - 1) };
                }
                0
            })
            .sum::<usize>();

        println!("{} result", result);
    } else {
        println!("File not OK");
    }
}

pub fn solve_4b() {
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input4.txt") {
        let mut result: Vec<(u32, usize)> = lines
            .map(|l| {
                if let Ok(ip) = l {
                    let parts: Vec<&str> = ip.split([':', '|']).collect();
                    let player_numbers = parts
                        .get(2)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<u32>().unwrap());
                    let winning_numbers: Vec<u32> = parts
                        .get(1)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    return (
                        1,
                        player_numbers
                            .filter(|x| winning_numbers.contains(x))
                            .count(),
                    );
                }
                (0, 0)
            })
            .collect();

        for i in 0..result.len() {
            println!("Card {} has {} matching numbers", i, result[i].1);
            if result[i].1 <= 0 {
                continue;
            }
            let number_of_wins = usize::try_from(result[i].1).unwrap();
            for _ in 0..result[i].0 {
                for j in i + 1..(i + number_of_wins + 1) {
                    if j > result.len() {
                        continue;
                    }

                    result[j].0 = result[j].0 + 1;
                }
            }
        }

        let total = result.iter().map(|x| x.0).sum::<u32>();
        println!("Result: {}", total);
    } else {
        println!("File not OK");
    }
}
