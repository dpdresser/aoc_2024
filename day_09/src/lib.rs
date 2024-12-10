use std::{collections::HashSet, fs};

pub fn load_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn checksum(disk_map: &str) -> usize {
    let mut block_representation = Vec::new();

    disk_map.char_indices().for_each(|(i, c)| {
        if i % 2 == 0 {
            let block_size = c.to_digit(10).unwrap();
            for _ in 0..block_size {
                block_representation.push((i / 2).to_string());
            }
        } else {
            let block_size = c.to_digit(10).unwrap();
            for _ in 0..block_size {
                block_representation.push(".".to_string());
            }
        }
    });

    let mut i = 0;
    let mut j = block_representation.len() - 1;

    while i < j {
        if block_representation[i] == "." {
            while j > i && block_representation[j] == "." {
                j -= 1;
            }
            if i < j {
                block_representation.swap(i, j);
            }
        }
        i += 1;
    }

    block_representation
        .iter()
        .enumerate()
        .map(|(i, value)| {
            if let Ok(number) = value.parse::<usize>() {
                i * number
            } else {
                0
            }
        })
        .sum()
}

pub fn checksum_part_2(disk_map: &str) -> usize {
    let mut block_representation = Vec::new();

    disk_map.char_indices().for_each(|(i, c)| {
        if i % 2 == 0 {
            let block_size = c.to_digit(10).unwrap();
            for _ in 0..block_size {
                block_representation.push((i / 2).to_string());
            }
        } else {
            let block_size = c.to_digit(10).unwrap();
            for _ in 0..block_size {
                block_representation.push(".".to_string());
            }
        }
    });

    let mut j = block_representation.len() - 1;
    let mut attempted_swapped_files = HashSet::new();

    while j > 0 {
        if block_representation[j] == "." {
            j -= 1;
            continue;
        } else if let Ok(number) = block_representation[j].parse::<usize>() {
            if attempted_swapped_files.contains(&number) {
                j -= 1;
                continue;
            } else {
                attempted_swapped_files.insert(number);
            }

            let number_count = block_representation
                .iter()
                .filter(|num| {
                    if let Ok(n) = num.parse::<usize>() {
                        n == number
                    } else {
                        false
                    }
                })
                .count();

            let mut free_space_start = None;
            let mut free_space_count = 0;

            for (i, value) in block_representation.iter().enumerate() {
                if i >= j {
                    break;
                }
                if value == "." {
                    if free_space_start.is_none() {
                        free_space_start = Some(i);
                    }
                    free_space_count += 1;
                    if free_space_count >= number_count {
                        break;
                    }
                } else {
                    free_space_start = None;
                    free_space_count = 0;
                }
            }

            if let Some(start) = free_space_start {
                for k in 0..number_count {
                    if number_count <= free_space_count {
                        block_representation.swap(start + k, j);
                        j -= 1;
                        if j == 0 {
                            break;
                        }
                    } else {
                        j -= 1;
                        if j == 0 {
                            break;
                        }
                    }
                }
            } else {
                j -= 1;
            }
        }
    }

    block_representation
        .iter()
        .enumerate()
        .map(|(i, value)| {
            if let Ok(number) = value.parse::<usize>() {
                i * number
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_1_simple() {
        let input = load_input("input_simple.txt");
        assert_eq!(checksum(&input), 1928);
    }

    #[test]
    fn day_09_part_1() {
        let input = load_input("input.txt");
        assert_eq!(checksum(&input), 6607511583593);
    }

    #[test]
    fn day_09_part_2_simple() {
        let input = load_input("input_simple.txt");
        assert_eq!(checksum_part_2(&input), 2858);
    }

    #[test]
    fn day_09_part_2() {
        let input = load_input("input.txt");
        assert_eq!(checksum_part_2(&input), 6636608781232);
    }
}
