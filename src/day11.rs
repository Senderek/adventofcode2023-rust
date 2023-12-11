use crate::utils;

const EXPANSION: usize = 1000000 - 1;

pub fn solve_11() {
    let lines = utils::read_lines("C:/rep/adventofcode2023/src/inputs/input11.txt").unwrap();
    let mut galaxies: Vec<(usize, usize)> = [].to_vec();
    let mut max_columns: usize = 0;
    let map: Vec<_> = lines
        .enumerate()
        .map(|(y, x)| {
            let line = x.unwrap();
            let chars: std::str::Chars<'_> = line.chars();
            if y == 0 {
                max_columns = line.len();
            }

            let row_galaxies = chars
                .into_iter()
                .enumerate()
                .filter(|(_, char)| char == &'#')
                .map(|x| x.0)
                .collect::<Vec<_>>();

            for x in row_galaxies {
                galaxies.push((x, y));
            }
            return line.chars().collect::<Vec<_>>();
        })
        .collect();

    let max_rows = map.len();

    let mut rows_without_galaxies = [].to_vec();
    for y in 0..max_columns {
        if !galaxies.iter().any(|(_, g_y)| g_y == &y) {
            rows_without_galaxies.push(y);
        }
    }
    let mut columns_without_galaxies = [].to_vec();
    for x in 0..max_rows {
        if !galaxies.iter().any(|(g_x, _)| g_x == &x) {
            columns_without_galaxies.push(x);
        }
    }

    println!("Columns: {:?}", rows_without_galaxies);
    println!("Rows: {:?}", columns_without_galaxies);
    println!("_____________________________________________________________");
    println!("Galaxies: {:?}", galaxies);

    galaxies = galaxies
        .iter()
        .map(|(x, y)| {
            let x_shift = columns_without_galaxies.iter().filter(|q| q < &&x).count();
            let y_shift = rows_without_galaxies.iter().filter(|q| q < &&y).count();
            return (*x + (x_shift * EXPANSION), *y + (y_shift * EXPANSION));
        })
        .collect();

    println!("_____________________________________________________________");
    println!("Galaxies after: {:?}", galaxies);
    println!("_____________________________________________________________");
    let galaxies_len = galaxies.len();
    let mut sum = 0;
    for i in 0..galaxies_len
    {
        let i_point = galaxies.get(i).unwrap();
        for j in i..galaxies_len
        {
            if j == i {
                continue;
            }

            let j_point = galaxies.get(j).unwrap();
            let distance = find_dist(i_point, j_point);
            // println!("{:?} -> {:?} = {}", i_point, j_point, distance);
            sum = sum + distance;
  
        }
    }
    println!("Sum: {}", sum);

}

fn find_dist((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> usize {
    let x_steps = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let y_steps = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    return x_steps + y_steps;
}
