use itertools::{repeat_n, Itertools};
use std::fs;

pub fn load_input(file_path: &str) -> Vec<Equation> {
    let text = fs::read_to_string(file_path).expect("Could not read from file");
    let mut equations = Vec::new();

    for line in text.lines() {
        let mut line_iter = line.split_terminator(':');
        let equation = Equation {
            test_value: line_iter.next().unwrap().parse().unwrap(),
            numbers: line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect(),
        };
        equations.push(equation);
    }

    equations
}

#[derive(Clone, Debug)]
pub struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

pub fn mul(a: u64, b: u64) -> u64 {
    a * b
}

pub fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

pub fn find_calibrated_equations(
    equations: &[Equation],
    operations: &[fn(u64, u64) -> u64],
) -> u64 {
    let mut value = 0;

    for equation in equations {
        let mut value_matches = false;
        let length = equation.numbers.len() - 1;
        for operation in repeat_n(operations.iter(), length).multi_cartesian_product() {
            if value_matches {
                break;
            }

            let mut operation_value = equation.numbers[0];

            for (&number, op) in equation.numbers[1..].iter().zip(operation.iter()) {
                operation_value = op(operation_value, number);
                if operation_value > equation.test_value {
                    break;
                }
            }

            if operation_value == equation.test_value {
                value += operation_value;
                value_matches = true;
            }
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_part_1_simple() {
        let equations = load_input("input_simple.txt");
        let operations = vec![add, mul];
        assert_eq!(find_calibrated_equations(&equations, &operations), 3749);
    }

    #[test]
    fn day_07_part_1() {
        let equations = load_input("input.txt");
        let operations = vec![add, mul];
        assert_eq!(
            find_calibrated_equations(&equations, &operations),
            882304362421
        );
    }

    #[test]
    fn day_07_part_2_simple() {
        let equations = load_input("input_simple.txt");
        let operations = vec![add, mul, concat];
        assert_eq!(find_calibrated_equations(&equations, &operations), 11387);
    }

    #[test]
    fn day_07_part_2() {
        let equations = load_input("input.txt");
        let operations = vec![add, mul, concat];
        assert_eq!(
            find_calibrated_equations(&equations, &operations),
            145149066755184
        );
    }
}
