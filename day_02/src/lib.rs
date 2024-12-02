use std::error::Error;
use std::fs;

pub fn is_safe(report: &Vec<i32>) -> bool {
    let safe_positive = [1, 2, 3];
    let safe_negative = [-1, -2, -3];

    let differences: Vec<i32> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let is_safe_positive = differences.iter().all(|item| safe_positive.contains(item));
    let is_safe_negative = differences.iter().all(|item| safe_negative.contains(item));

    if is_safe_positive || is_safe_negative {
        return true;
    } else {
        return false;
    }
}

pub fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    match is_safe(&report) {
        true => return true,
        false => {
            for i in 0..report.len() {
                let mut temp_report = report.clone();
                temp_report.remove(i);
                if is_safe(&temp_report) {
                    return true;
                }
            }
            return false;
        }
    }
}

pub fn count_safe_rows(apply_dampener: bool) -> Result<i32, Box<dyn Error>> {
    let file_path = "input.txt";

    let text = fs::read_to_string(file_path).expect("Could not read from file");

    let count_safe = text
        .lines()
        .filter(|line| {
            let mut line_slice: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            match apply_dampener {
                false => is_safe(&mut line_slice),
                true => is_safe_with_dampener(&mut line_slice),
            }
        })
        .count() as i32;

    Ok(count_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_test_is_safe_simple() {
        let report1 = vec![7, 6, 4, 2, 1];
        let report2 = vec![1, 2, 7, 8, 9];
        let report3 = vec![9, 7, 6, 2, 1];
        let report4 = vec![1, 3, 2, 4, 5];
        let report5 = vec![8, 6, 4, 4, 1];
        let report6 = vec![1, 3, 6, 7, 9];

        assert_eq!(is_safe(&report1), true);
        assert_eq!(is_safe(&report2), false);
        assert_eq!(is_safe(&report3), false);
        assert_eq!(is_safe(&report4), false);
        assert_eq!(is_safe(&report5), false);
        assert_eq!(is_safe(&report6), true);
    }

    #[test]
    fn day_02_test_is_safe_input() {
        assert_eq!(count_safe_rows(false).unwrap(), 502);
    }

    #[test]
    fn day_02_test_is_safe_simple_with_dampener() {
        let report1 = vec![7, 6, 4, 2, 1];
        let report2 = vec![1, 2, 7, 8, 9];
        let report3 = vec![9, 7, 6, 2, 1];
        let report4 = vec![1, 3, 2, 4, 5];
        let report5 = vec![8, 6, 4, 4, 1];
        let report6 = vec![1, 3, 6, 7, 9];

        assert_eq!(is_safe_with_dampener(&report1), true);
        assert_eq!(is_safe_with_dampener(&report2), false);
        assert_eq!(is_safe_with_dampener(&report3), false);
        assert_eq!(is_safe_with_dampener(&report4), true);
        assert_eq!(is_safe_with_dampener(&report5), true);
        assert_eq!(is_safe_with_dampener(&report6), true);
    }

    #[test]
    fn day_02_test_is_safe_input_with_dampener() {
        assert_eq!(count_safe_rows(true).unwrap(), 544);
    }
}
