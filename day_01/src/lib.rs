use std::error::Error;
use std::fs;

pub fn find_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> Result<i32, Box<dyn Error>> {
    if left.len() != right.len() {
        return Err("Vectors must be of the same length".into());
    }

    left.sort();
    right.sort();

    let distance = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (right - left).abs())
        .sum();

    Ok(distance)
}

pub fn calculate_similarity(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, Box<dyn Error>> {
    if left.len() != right.len() {
        return Err("Vectors must be of the same length".into());
    }

    let similarity = left
        .iter()
        .map(|left| right.iter().filter(|right| *right == left).count() as i32 * *left)
        .sum();

    Ok(similarity)
}

pub fn load_text_vectors() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let file_path = "input.txt";

    let text = fs::read_to_string(file_path).expect("Could not read from file");
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in text.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(left_num), Some(right_num)) = (numbers.next(), numbers.next()) {
            left.push(left_num.parse()?);
            right.push(right_num.parse()?);
        }
    }

    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_test_distance_simple() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(find_distance(&mut left, &mut right).unwrap(), 11);
    }

    #[test]
    fn day_01_test_not_equal_lengths() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3];
        assert!(find_distance(&mut left, &mut right).is_err());
    }

    #[test]
    fn day_01_test_distance_with_input() {
        let (mut left, mut right) = load_text_vectors().unwrap();
        assert_eq!(find_distance(&mut left, &mut right).unwrap(), 1941353);
    }

    #[test]
    fn day_01_test_similarity_simple() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_similarity(&mut left, &mut right).unwrap(), 31);
    }

    #[test]
    fn day_01_test_similarity_with_input() {
        let (mut left, mut right) = load_text_vectors().unwrap();
        assert_eq!(
            calculate_similarity(&mut left, &mut right).unwrap(),
            22539317
        );
    }
}
