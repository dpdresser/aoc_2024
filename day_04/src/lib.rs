use ndarray::{Array2, Axis};
use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub fn load_input() -> String {
    let file_path = "input.txt";
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn string_to_ndarray(input: &str) -> Array2<char> {
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

pub fn search_array(word: &str, array: &Array2<char>) -> usize {
    let n = array.len_of(Axis(0));
    let m = array.len_of(Axis(1));
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            if array[[i, j]] == word.chars().next().unwrap() {
                for &(dir_x, dir_y) in &DIRECTIONS {
                    let mut found = true;
                    for (k, letter) in word_chars.iter().enumerate() {
                        let x = i as i32 + k as i32 * dir_x;
                        let y = j as i32 + k as i32 * dir_y;

                        if x < 0
                            || x >= n as i32
                            || y < 0
                            || y >= m as i32
                            || array[[x as usize, y as usize]] != *letter
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn search_array_part_2(array: &Array2<char>) -> usize {
    let n = array.len_of(Axis(0));
    let m = array.len_of(Axis(1));
    let mut count = 0;

    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            let letter_1 = array[[i - 1, j - 1]];
            let letter_2 = array[[i, j]];
            let letter_3 = array[[i + 1, j + 1]];

            let letter_4 = array[[i - 1, j + 1]];
            let letter_5 = array[[i + 1, j - 1]];

            let pattern = [('M', 'A', 'S'), ('S', 'A', 'M')];
            for down in pattern {
                for up in pattern {
                    if (letter_1, letter_2, letter_3) == down
                        && (letter_4, letter_2, letter_5) == up
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_test_part1_simple() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = string_to_ndarray(&input);
        assert_eq!(search_array("XMAS", &grid), 18);
    }

    #[test]
    fn day_04_test_part1() {
        let input = load_input();
        let grid = string_to_ndarray(&input);
        assert_eq!(search_array("XMAS", &grid), 2358);
    }

    #[test]
    fn day_04_test_part2_simple() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = string_to_ndarray(&input);
        assert_eq!(search_array_part_2(&grid), 9);
    }

    #[test]
    fn day_04_test_part2() {
        let input = load_input();
        let grid = string_to_ndarray(&input);
        assert_eq!(search_array_part_2(&grid), 1737);
    }
}
