use count_digits::CountDigits;
use counter::Counter;
use std::fs;

pub fn load_input(file_path: &str) -> Vec<usize> {
    let text = fs::read_to_string(file_path).expect("Could not read from file");
    let mut numbers = Vec::new();
    text.split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|n| numbers.push(n.parse::<usize>().unwrap()));

    numbers
}

fn string_and_split(num: usize) -> [usize; 2] {
    let num_digits = (num as f64).log10().floor() as usize + 1;
    let half_digits = num_digits / 2;
    let divisor = 10_usize.pow(half_digits as u32);
    let first_half = num / divisor;
    let second_half = num % divisor;

    [first_half, second_half]
}

fn transform_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.count_digits() % 2 == 0 {
        string_and_split(stone).to_vec()
    } else {
        vec![stone * 2024]
    }
}

pub fn blink(stones: &[usize], num_blinks: usize) -> usize {
    let mut stones = stones.iter().cloned().collect::<Counter<_>>();
    let mut next_stones: Counter<usize> = Counter::new();
    for _ in 0..num_blinks {
        next_stones.clear();
        for (&stone, &count) in stones.iter() {
            let transformed = transform_stone(stone);
            for new_stone in transformed {
                next_stones[&new_stone] += count;
            }
        }
        std::mem::swap(&mut stones, &mut next_stones);
    }
    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_part_1_simple() {
        let stones = vec![125, 17];
        assert_eq!(blink(&stones, 6), 22);
        assert_eq!(blink(&stones, 25), 55312);
    }

    #[test]
    fn day_11_part_1() {
        let stones = load_input("input.txt");
        assert_eq!(blink(&stones, 25), 194782);
    }

    #[test]
    fn day_11_part_2() {
        let stones = load_input("input.txt");
        assert_eq!(blink(&stones, 75), 233007586663131);
    }
}
