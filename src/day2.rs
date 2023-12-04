use crate::utils;

#[allow(dead_code)]
pub fn solve_2a() {
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input2.txt") {
        let result = lines
            .into_iter()
            .enumerate()
            .map(|(xind, l)| {
                if let Ok(ip) = l {
                    let digs = xind.to_string().as_str().len();
                    let (id, game) = ip.split_at(7 + digs);
                    for part in game.split([',', ';']) {
                        let part_split = part.trim().split_once(' ');
                        if let Some((nmb, color)) = part_split {
                            let number_of_cubes = nmb.parse::<u32>().unwrap();
                            match color {
                                "red" => {
                                    if number_of_cubes > 12 {
                                        println!("{}impossible", id);
                                        return 0;
                                    }
                                }
                                "green" => {
                                    if number_of_cubes > 13 {
                                        println!("{}impossible", id);
                                        return 0;
                                    }
                                }
                                "blue" => {
                                    if number_of_cubes > 14 {
                                        println!("{}impossible", id);
                                        return 0;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    println!("{}OK", id);
                    return xind as u32 + 1;
                }
                0
            })
            .sum::<u32>();
        println!("{} result", result);
    } else {
        println!("File not OK");
    }
}

#[allow(dead_code)]
pub fn solve_2b() {
    if let Ok(lines) = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input2.txt") {
        let result = lines
            .into_iter()
            .enumerate()
            .map(|(xind, l)| {
                if let Ok(ip) = l {
                    let digs = xind.to_string().as_str().len();
                    let (id, game) = ip.split_at(7 + digs);
                    let mut red_power = 0;
                    let mut green_power = 0;
                    let mut blue_power = 0;
                    for part in game.split([',', ';']) {
                        let part_split = part.trim().split_once(' ');
                        if let Some((nmb, color)) = part_split {
                            let number_of_cubes = nmb.parse::<u32>().unwrap();
                            match color {
                                "red" => {
                                    if number_of_cubes > red_power {
                                        red_power = number_of_cubes
                                    }
                                }
                                "green" => {
                                    if number_of_cubes > green_power {
                                        green_power = number_of_cubes
                                    }
                                }
                                "blue" => {
                                    if number_of_cubes > blue_power {
                                        blue_power = number_of_cubes
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    let pwr = red_power * green_power * blue_power;
                    println!("{} {}", id, pwr);
                    return pwr;
                }
                0
            })
            .sum::<u32>();
        println!("{} result", result);
    } else {
        println!("File not OK");
    }
}
