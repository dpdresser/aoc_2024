use std::fs;

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
        assert_eq!(checksum(&input), 2858);
    }
}
