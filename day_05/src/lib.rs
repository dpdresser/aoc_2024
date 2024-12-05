use std::collections::HashMap;
use std::fs;

pub fn load_input(input_file: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let text = fs::read_to_string(input_file).expect("Could not read from file");
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in text.lines() {
        if line.contains("|") {
            let mut line_iter = line.split_terminator("|");
            rules
                .entry(line_iter.next().unwrap().parse().unwrap())
                .or_insert_with(Vec::new)
                .push(line_iter.next().unwrap().parse().unwrap());
        } else if line.contains(",") {
            let line_vec: Vec<usize> = line
                .split_terminator(",")
                .map(|num| num.parse().unwrap())
                .collect();

            updates.push(line_vec);
        }
    }

    (rules, updates)
}

pub fn sum_middle_of_updates_in_right_order(
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
) -> usize {
    let mut correct_middle_entries = Vec::new();

    for update in updates {
        let mut correct_count = 0;
        for i in 0..(update.len() - 1) {
            if let Some(rule) = rules.get(&update[i]) {
                for page in update.iter().skip(i + 1) {
                    if rule.contains(page) {
                        correct_count += 1;
                    }
                }
            }
        }

        if correct_count == (1..update.len() as i32).sum::<i32>() as usize {
            correct_middle_entries.push(update[update.len() / 2])
        }
    }

    correct_middle_entries.iter().sum()
}

pub fn sum_middle_of_updates_in_wrong_order(
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
) -> usize {
    let mut wrong_middle_entries = Vec::new();

    for mut update in updates {
        let mut correct_count = 0;
        for i in 0..(update.len() - 1) {
            if let Some(rule) = rules.get(&update[i]) {
                for page in update.iter().skip(i + 1) {
                    if rule.contains(page) {
                        correct_count += 1;
                    }
                }
            }
        }

        if correct_count != (1..update.len() as i32).sum::<i32>() as usize {
            let mut reordered_update = Vec::new();

            while !update.is_empty() {
                'outer: for (i, page) in update.iter().enumerate() {
                    let mut other_page_count = 0;
                    for (j, other_page) in update.iter().enumerate() {
                        if j != i {
                            if let Some(rule) = rules.get(other_page) {
                                if rule.contains(page) {
                                    other_page_count += 1;
                                }
                            }
                        }
                    }

                    if other_page_count == update.len() - 1 {
                        reordered_update.insert(0, update.remove(i));
                        break 'outer;
                    }
                }
            }

            if !reordered_update.is_empty() {
                wrong_middle_entries.push(reordered_update[reordered_update.len() / 2]);
            }
        }
    }

    wrong_middle_entries.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_part_1_simple() {
        let (rules, updates) = load_input("input_simple.txt");
        assert_eq!(sum_middle_of_updates_in_right_order(rules, updates), 143);
    }

    #[test]
    fn day_05_part_1() {
        let (rules, updates) = load_input("input.txt");
        assert_eq!(sum_middle_of_updates_in_right_order(rules, updates), 5108);
    }

    #[test]
    fn day_05_part_2_simple() {
        let (rules, updates) = load_input("input_simple.txt");
        assert_eq!(sum_middle_of_updates_in_wrong_order(rules, updates), 123);
    }

    #[test]
    fn day_05_part_2() {
        let (rules, updates) = load_input("input.txt");
        assert_eq!(sum_middle_of_updates_in_wrong_order(rules, updates), 7380);
    }
}
