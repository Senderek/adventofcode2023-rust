use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use utf8_chars::BufReadCharsExt;

#[allow(dead_code)]
pub fn solve_15() {
    let mut f = BufReader::new(
        File::open("C:/rep/adventofcode2023/src/inputs/input15.txt").expect("open failed"),
    );

    let mut sum: i64 = 0;
    let mut cur_val: i64 = 0;

    for c in f.chars().map(|x| x.unwrap()) {
        if c == ',' {
            sum = sum + cur_val;
            cur_val = 0;
        } else {
            if c == '\n' {
                continue;
            }
            cur_val = ((cur_val + c as i64) * 17) % 256;
        }
    }

    println!("Result: {}", sum + cur_val);
}

const RADIX: u32 = 10;

enum Sign {
    Unknown,
    Dash,
    Equals,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sign::Unknown => write!(f, "Unknown"),
            Sign::Dash => write!(f, "-"),
            Sign::Equals => write!(f, "="),
        }
    }
}

pub fn solve_15b() {
    let mut f = BufReader::new(
        File::open("C:/rep/adventofcode2023/src/inputs/input15.txt").expect("open failed"),
    );

    let mut boxes: Vec<HashMap<String, (u32, u32)>> = vec![HashMap::new(); 256];
    let mut code: Vec<char> = vec![];
    let mut sign: Sign = Sign::Unknown;
    let mut focal_length: u32 = 0;

    for c in f.chars().map(|x| x.unwrap()) {
        if c == '\n' {
            continue;
        }

        if matches!(sign, Sign::Unknown) {
            if c == '=' {
                sign = Sign::Equals;
            } else if c == '-' {
                sign = Sign::Dash;
            } else {
                code.push(c);
            }
        } else {
            if c == ',' {
                let (l, v) = label_to_value(code.clone());
                // println!(
                //     "Code: {}. Val: {} Sign: {}, Focal length: {}",
                //     l, v, sign, focal_length
                // );
                // println!("After \"{}{}{}\"", l, sign, focal_length);
                let cur_box = boxes.get_mut(v).unwrap();
                match sign {
                    Sign::Dash => {
                        (*cur_box).remove(&(l.clone()));
                    }
                    Sign::Equals => {
                        if cur_box.contains_key(&(l.clone())) {
                            cur_box.entry(l.clone()).and_modify(|h| {
                                h.0 = focal_length;
                            });
                        } else {
                            let mn = cur_box.iter().map(|y| y.1 .1).max();
                            let new_index = match mn {
                                Some(s) => s + 1,
                                None => 0,
                            };
                            (*cur_box).insert(l, (focal_length, new_index));
                        }
                    }
                    _ => panic!(),
                }

                code = vec![];
                focal_length = 0;
                sign = Sign::Unknown;
                // boxes
                // .iter()
                // .enumerate()
                // .filter(|(_, x)| !x.is_empty())
                // .for_each(|(i, x)| { println!("Box {}: {:?}", i, x) })
            } else {
                focal_length = c.to_digit(RADIX).unwrap();
            }
        }
    }
    let (l, v) = label_to_value(code.clone());
    // println!("After \"{}{}{}\"", l, sign, focal_length);
    let cur_box = boxes.get_mut(v).unwrap();
    match sign {
        Sign::Dash => {
            (*cur_box).remove(&(l.clone()));
        }
        Sign::Equals => {
            if cur_box.contains_key(&(l.clone())) {
                cur_box.entry(l.clone()).and_modify(|h| {
                    h.0 = focal_length;
                });
            } else {
                let mn = cur_box.iter().map(|y| y.1 .1).max();
                let new_index = match mn {
                    Some(s) => s + 1,
                    None => 0,
                };
                (*cur_box).insert(l, (focal_length, new_index));
            }
        }
        _ => panic!(),
    }

    // boxes
    // .iter()
    // .enumerate()
    // .filter(|(_, x)| !x.is_empty())
    // .for_each(|(i, x)| { println!("Box {}: {:?}", i, x) });

    let result: usize = boxes
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.is_empty())
        .map(|(box_indx, x)| {
            let mut all_values: Vec<(u32, u32)> = x.values().cloned().collect();
            all_values.sort_by_key(|k| k.1);
            return (box_indx, all_values.iter().map(|t| t.0).collect::<Vec<_>>());
        })
        .map(|(box_number, box_content)| {
            let box_val = 1 + box_number;
            box_content
                .iter()
                .enumerate()
                .map(|(slot_number, slot_focal_length)| {
                    let usize_slot_focal_length = usize::try_from(*slot_focal_length).unwrap();
                    // let box_r = box_val * (slot_number + 1) * usize_slot_focal_length;
                    // println!(
                    //     "{} * {} * {} = {}",
                    //     box_val,
                    //     (slot_number + 1),
                    //     usize_slot_focal_length,
                    //     box_r
                    // );
                    box_val * (slot_number + 1) * usize_slot_focal_length
                })
                .sum::<usize>()
        })
        .sum();
    println!("Result: {}", result);
}

fn label_to_value(label: Vec<char>) -> (String, usize) {
    let v = label
        .iter()
        .fold(0, |cur_val, c| ((cur_val + *c as usize) * 17) % 256);
    return (label.iter().collect::<String>(), v);
}
