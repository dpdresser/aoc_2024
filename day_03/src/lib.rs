use regex::Regex;
use std::fs;

pub fn load_input() -> String {
    let file_path = "input.txt";
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn parse_text_with_regex(text: &str) -> usize {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    re.captures_iter(text)
        .map(|slice| {
            let a: usize = slice.name("a").unwrap().as_str().parse().unwrap();
            let b: usize = slice.name("b").unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

pub fn parse_text_conditional_with_regex(text: &str) -> usize {
    // Regular expressions for enable / disable flags and multiply commands
    let re_enable = Regex::new(r"do\(\)").unwrap();
    let re_disable = Regex::new(r"don't\(\)").unwrap();

    // Create iterators for all regex matches
    let enable_iter = re_enable.find_iter(text);
    let mut disable_iter = re_disable.find_iter(text);

    // Initialize sum
    let mut sum = 0;
    let mut slice_start = 0;
    let mut slice_end = 0;

    // First iteration
    if let Some(disable_match) = disable_iter.next() {
        slice_end = disable_match.start();
        let text_slice = &text[slice_start..slice_end];
        sum += parse_text_with_regex(text_slice);
    }

    // Iterate until all enable conditionals have been used, starting
    // with first enable match were the start > disable match end
    'enable_iter: for enable_match in enable_iter {
        // If has not reached current slice end, continue
        if enable_match.start() < slice_end {
            continue 'enable_iter;
        }

        // Update slice_start
        slice_start = enable_match.end();

        // Find first disable match with start greater than current
        // enable match end
        for disable_match in disable_iter.by_ref() {
            if disable_match.start() > slice_start {
                slice_end = disable_match.start();
                let text_slice = &text[slice_start..slice_end];
                sum += parse_text_with_regex(text_slice);
                continue 'enable_iter;
            }
        }

        // If disable iter used up, use remaining slice and end the while loop
        let text_slice = &text[slice_start..];
        sum += parse_text_with_regex(text_slice);
        break;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_test_simple_part_1() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_text_with_regex(text), 161);
    }

    #[test]
    fn day_03_test_part_1() {
        let text = &load_input();
        assert_eq!(parse_text_with_regex(text), 183380722);
    }

    #[test]
    fn day_03_test_simple_part_2() {
        let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_text_conditional_with_regex(text), 48);
    }

    #[test]
    fn day_03_test_part_2() {
        let text = &load_input();
        assert_eq!(parse_text_conditional_with_regex(text), 82733683);
    }
}
