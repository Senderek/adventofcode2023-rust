use crate::utils;

pub fn solve_12() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input12.txt").unwrap();
    let map: Vec<_> = lines
        .map(|line| {
            let line = line.unwrap();
            let (condition_records, condition_numbers) = line.split_once(' ').unwrap();
            let cn = condition_numbers
                .trim()
                .split(',')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            return (String::from(condition_records), cn);
        })
        .collect();
    let s: usize = map
        .iter()
        .enumerate()
        .map(|(i, (x, correct_arrangment))| {
            let permutations = fill_question_marks(&x);
            // println!("__________________________________________________________________________________________________");
            // println!("{} permutations: {:?}", x, permutations);
            let arrangments = permutations
                .iter()
                .filter(|x| {
                    let permutation_arrangment: Vec<_> = x
                        .split('.')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.len())
                        .collect();
                    // println!(
                    //     "{}: {:?} {:?} {}",
                    //     x,
                    //     x.split('.').filter(|s| !s.is_empty()).collect::<Vec<_>>(),
                    //     permutation_arrangment,
                    //     do_vecs_match(&permutation_arrangment, &correct_arrangment)
                    // );
                    return do_vecs_match(&permutation_arrangment, &correct_arrangment);
                })
                .count();
            println!("{} arrangments: {}", i, arrangments);
            return arrangments;
        })
        .sum();

    println!("Total: {}", s);
}

fn fill_question_marks(s: &str) -> Vec<String> {
    let pound_count = s.chars().filter(|&c| c == '?').count();
    let mut permutations = vec![];
    for product in permutations_of_dot_and_hash(pound_count) {
        let mut iter = product.chars().into_iter();
        let filled_string = s
            .chars()
            .map(|c| if c == '?' { iter.next().unwrap() } else { c })
            .collect::<String>();
        permutations.push(filled_string);
    }

    permutations
}

fn permutations_of_dot_and_hash(length: usize) -> Vec<String> {
    fn generate_permutations(chars: &mut Vec<char>, index: usize, result: &mut Vec<String>) {
        if index == chars.len() {
            result.push(chars.iter().collect());
            return;
        }

        for &c in ['.', '#'].iter() {
            chars[index] = c;
            generate_permutations(chars, index + 1, result);
        }
    }

    let mut chars = vec!['#'; length];
    let mut result = Vec::new();

    generate_permutations(&mut chars, 0, &mut result);
    result
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
