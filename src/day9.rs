use crate::utils;

#[allow(dead_code)]
pub fn solve_9() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input9.txt").unwrap();
    let result = lines
        .map(|x| {
            let line = x.unwrap();
            let values = line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            return find_next(values);
        })
        .sum::<i32>();

    let lines2 = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input9.txt").unwrap();
    let result2 = lines2
        .map(|x| {
            let line = x.unwrap();
            let values = line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .rev()
                .collect::<Vec<i32>>();

            return find_next(values);
        })
        .sum::<i32>();

    println!("Result: {}. Result reversed: {}", result, result2);
}

fn find_next(vec: Vec<i32>) -> i32 {
    let mut next_vec: Vec<i32> = [].to_vec();

    if vec.iter().all(|x| x == &0) {
        return 0;
    }

    for i in 1..vec.len() {
        let cur = *(vec.get(i).clone().unwrap());
        let prev = vec.get(i - 1).clone().unwrap();

        let dif = cur - *prev;
        next_vec.push(i32::try_from(dif).unwrap());
    }

    let r = find_next(next_vec) + vec.last().unwrap();

    let as_str = vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    return r;
}
