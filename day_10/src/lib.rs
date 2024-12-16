use ndarray::Array2;
use std::collections::HashSet;
use std::fs;

const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn load_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn create_map(file_path: &str) -> Array2<usize> {
    let input = load_input(file_path);
    let rows: Vec<&str> = input.lines().collect();
    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut array = Array2::from_elem((nrows, ncols), 0);

    for (i, row) in rows.iter().enumerate() {
        for (j, letter) in row.chars().enumerate() {
            if letter == '.' {
                array[[i, j]] = 20;
            } else {
                array[[i, j]] = letter.to_string().parse::<usize>().unwrap();
            }
        }
    }

    array
}

pub fn find_trailheads(map: &Array2<usize>) -> HashSet<(usize, usize)> {
    let mut trailheads = HashSet::new();
    map.indexed_iter().for_each(|((row, col), &num)| {
        if num == 0 {
            trailheads.insert((row, col));
        }
    });

    trailheads
}

pub fn count_paths(map: &Array2<usize>, start_position: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut reached = HashSet::new();
    search_next_position(map, start_position, 0, &mut visited, &mut reached);

    reached.len()
}

pub fn rate_paths(map: &Array2<usize>, start_position: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut reached = HashSet::new();
    search_next_position(map, start_position, 0, &mut visited, &mut reached)
}

pub fn search_next_position(
    map: &Array2<usize>,
    current_position: (usize, usize),
    current_value: usize,
    visited: &mut HashSet<(usize, usize)>,
    reached: &mut HashSet<(usize, usize)>,
) -> usize {
    let (current_row, current_column) = current_position;

    if current_row >= map.nrows()
        || current_column >= map.ncols()
        || visited.contains(&current_position)
        || map[[current_row, current_column]] != current_value
    {
        return 0;
    }

    if current_value == 9 {
        reached.insert(current_position);
        return 1;
    }

    visited.insert(current_position);

    let mut paths = 0;
    let next_value = current_value + 1;

    for (move_row, move_column) in DIRECTIONS {
        let new_row = (current_row as i8 + move_row) as usize;
        let new_column = (current_column as i8 + move_column) as usize;
        paths += search_next_position(map, (new_row, new_column), next_value, visited, reached);
    }

    visited.remove(&current_position);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part_1_simple() {
        let map = create_map("input_simple.txt");
        let trailheads = find_trailheads(&map);
        let mut total_paths = 0;

        for &trailhead in &trailheads {
            total_paths += count_paths(&map, trailhead);
        }

        assert_eq!(total_paths, 36);
    }

    #[test]
    fn day_10_part_1() {
        let map = create_map("input.txt");
        let trailheads = find_trailheads(&map);
        let mut total_paths = 0;

        for &trailhead in &trailheads {
            total_paths += count_paths(&map, trailhead);
        }

        assert_eq!(total_paths, 778);
    }

    #[test]
    fn day_10_part_2_simple() {
        let map = create_map("input_simple.txt");
        let trailheads = find_trailheads(&map);
        let mut total_paths = 0;

        for &trailhead in &trailheads {
            total_paths += rate_paths(&map, trailhead);
        }

        assert_eq!(total_paths, 81);
    }

    #[test]
    fn day_10_part_2() {
        let map = create_map("input.txt");
        let trailheads = find_trailheads(&map);
        let mut total_paths = 0;

        for &trailhead in &trailheads {
            total_paths += rate_paths(&map, trailhead);
        }

        assert_eq!(total_paths, 1925);
    }
}
