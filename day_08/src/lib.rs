use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn create_map(file_path: &str) -> Array2<char> {
    let input = load_input(file_path);
    let rows: Vec<&str> = input.lines().collect();
    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut array = Array2::from_elem((nrows, ncols), ' ');

    for (i, row) in rows.iter().enumerate() {
        for (j, letter) in row.chars().enumerate() {
            array[[i, j]] = letter;
        }
    }

    array
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Location {
    x: usize,
    y: usize,
}

pub fn locate_antennas(antenna_map: &Array2<char>) -> HashMap<char, Vec<Location>> {
    let mut antenna_locations: HashMap<char, Vec<Location>> = HashMap::new();

    for ((i, j), &c) in antenna_map.indexed_iter() {
        if c != '.' {
            let new_location = Location { x: i, y: j };
            antenna_locations.entry(c).or_default().push(new_location);
        } else {
            continue;
        }
    }

    antenna_locations
}

pub fn find_antinodes(
    antenna_map: &Array2<char>,
    antenna_locations: &HashMap<char, Vec<Location>>,
) -> usize {
    let mut unique_antinodes = HashSet::new();

    for locations in antenna_locations.values() {
        let mut antinodes = HashSet::new();

        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let loc1 = &locations[i];
                let loc2 = &locations[j];

                let dx = loc2.x as isize - loc1.x as isize;
                let dy = loc2.y as isize - loc1.y as isize;

                let x3 = loc1.x as isize - dx;
                let y3 = loc1.y as isize - dy;
                let x4 = loc2.x as isize + dx;
                let y4 = loc2.y as isize + dy;

                if x3 >= 0
                    && x3 < antenna_map.nrows() as isize
                    && y3 >= 0
                    && y3 < antenna_map.ncols() as isize
                {
                    antinodes.insert(Location {
                        x: x3 as usize,
                        y: y3 as usize,
                    });
                }

                if x4 >= 0
                    && x4 < antenna_map.nrows() as isize
                    && y4 >= 0
                    && y4 < antenna_map.ncols() as isize
                {
                    antinodes.insert(Location {
                        x: x4 as usize,
                        y: y4 as usize,
                    });
                }
            }
        }

        unique_antinodes.extend(antinodes);
    }

    unique_antinodes.len()
}

pub fn find_antinodes_part_2(
    antenna_map: &Array2<char>,
    antenna_locations: &HashMap<char, Vec<Location>>,
) -> usize {
    let mut unique_antinodes = HashSet::new();

    for locations in antenna_locations.values() {
        let mut antinodes = HashSet::new();

        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let loc1 = &locations[i];
                let loc2 = &locations[j];

                let dx = loc2.x as isize - loc1.x as isize;
                let dy = loc2.y as isize - loc1.y as isize;

                let gcd = greatest_common_divisor(dx.abs(), dy.abs());
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                let mut x = loc1.x as isize;
                let mut y = loc1.y as isize;

                while x >= 0
                    && x < antenna_map.nrows() as isize
                    && y >= 0
                    && y < antenna_map.ncols() as isize
                {
                    antinodes.insert(Location {
                        x: x as usize,
                        y: y as usize,
                    });
                    x += step_x;
                    y += step_y;
                }

                x = loc1.x as isize - step_x;
                y = loc1.y as isize - step_y;

                while x >= 0
                    && x < antenna_map.nrows() as isize
                    && y >= 0
                    && y < antenna_map.ncols() as isize
                {
                    antinodes.insert(Location {
                        x: x as usize,
                        y: y as usize,
                    });
                    x -= step_x;
                    y -= step_y;
                }
            }
        }

        unique_antinodes.extend(antinodes);
    }

    unique_antinodes.len()
}

fn greatest_common_divisor(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_08_part_1_simple() {
        let antenna_map = create_map("input_simple.txt");
        let antenna_locations = locate_antennas(&antenna_map);
        assert_eq!(find_antinodes(&antenna_map, &antenna_locations), 14);
    }

    #[test]
    fn day_08_part_1() {
        let antenna_map = create_map("input.txt");
        let antenna_locations = locate_antennas(&antenna_map);
        assert_eq!(find_antinodes(&antenna_map, &antenna_locations), 276);
    }

    #[test]
    fn day_08_part_2_simple() {
        let antenna_map = create_map("input_simple.txt");
        let antenna_locations = locate_antennas(&antenna_map);
        assert_eq!(find_antinodes_part_2(&antenna_map, &antenna_locations), 34);
    }

    #[test]
    fn day_08_part_2() {
        let antenna_map = create_map("input.txt");
        let antenna_locations = locate_antennas(&antenna_map);
        assert_eq!(find_antinodes_part_2(&antenna_map, &antenna_locations), 991);
    }
}
