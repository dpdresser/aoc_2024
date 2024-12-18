use std::fs;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn load_input(file_path: &str) -> Vec<Vec<char>> {
    let text = fs::read_to_string(file_path).expect("Could not read from file");
    let mut map = Vec::new();

    for line in text.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    map
}

fn find_region(
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
    i: usize,
    j: usize,
    plant: char,
) -> (usize, usize) {
    let mut coordinates = vec![(i, j)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((x, y)) = coordinates.pop() {
        if visited[x][y] || map[x][y] != plant {
            continue;
        }

        visited[x][y] = true;
        area += 1;

        for (dx, dy) in DIRECTIONS.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x < 0
                || new_y < 0
                || new_x >= map.len() as isize
                || new_y >= map[0].len() as isize
                || map[new_x as usize][new_y as usize] != plant
            {
                perimeter += 1;
            } else if !visited[new_x as usize][new_y as usize] {
                coordinates.push((new_x as usize, new_y as usize));
            }
        }
    }
    (area, perimeter)
}

pub fn calculate_fencing_price(map: &[Vec<char>]) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut total_price = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited[i][j] {
                let (area, perimeter) = find_region(map, &mut visited, i, j, map[i][j]);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_part_1_simple() {
        let map = vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ];
        assert_eq!(calculate_fencing_price(&map), 140);

        let map = vec![
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
        ];
        assert_eq!(calculate_fencing_price(&map), 772);

        let map = load_input("input_simple.txt");
        assert_eq!(calculate_fencing_price(&map), 1930);
    }

    #[test]
    fn day_12_part_1() {
        let map = load_input("input.txt");
        assert_eq!(calculate_fencing_price(&map), 1550156);
    }
}
