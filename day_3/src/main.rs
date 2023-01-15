use std::{ collections::{ HashMap }, fs };

fn main() {
    //     let rucksacks =
    //         "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    // ";

    let rucksacks = fs::read_to_string("./input.txt").unwrap();

    let sum_of_rucksack_anomalies = rucksack_compartment_differentiation(&rucksacks);
    println!("{}", sum_of_rucksack_anomalies);

    let sum_of_rucksack_groups_priority = rucksack_group_identification(&rucksacks);
    println!("{}", sum_of_rucksack_groups_priority)
}

fn rucksack_compartment_differentiation(rucksacks: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in rucksacks.lines() {
        let (c1, c2) = &line.split_at(&line.len() / 2);

        let c2_bytes = c2.as_bytes();

        let mut chars = HashMap::new();

        for item in c1.as_bytes() {
            if !c2_bytes.contains(item) {
                continue;
            }
            if let Some(priority) = determine_byte_value(&item) {
                chars.insert(item, priority);
            }
        }
        let line_sum: i32 = chars.values().sum();
        sum += line_sum;
    }

    sum
}
#[test]
fn test_rucksack_compartment_differentiation() {
    let test_cases = vec![
        ("vJrwpWtwJgWrhcsFMMfFFhFp", 16),
        ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38),
        ("PmmdzqPrVvPwwTWBwg", 42),
        ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22),
        ("ttgJtRGJQctTZtZT", 20),
        ("CrZsJsPPZsGzwwsLwLmpwMDw", 19)
    ];

    for (input, output) in test_cases {
        assert_eq!(rucksack_compartment_differentiation(input), output);
    }
}

fn rucksack_group_identification(rucksacks: &str) -> i32 {
    let mut sum = 0;

    for (index, line) in rucksacks.lines().enumerate().step_by(3) {
        let second_line = rucksacks
            .lines()
            .nth(index + 1)
            .unwrap();
        let mut second_line_matching = HashMap::new();
        for byte in line.as_bytes() {
            if second_line.contains(|char| (*byte as char) == char) {
                second_line_matching.insert(byte, true);
            }
        }

        let third_line = rucksacks
            .lines()
            .nth(index + 2)
            .unwrap();
        let mut third_line_matching = HashMap::new();
        for byte in second_line_matching.keys() {
            if third_line.contains(**byte as char) {
                third_line_matching.insert(byte, true);
            }
        }

        for char in third_line_matching.keys() {
            if let Some(priority) = determine_byte_value(&char) {
                sum += priority;
            }
        }
    }

    sum
}
#[test]
fn test_rucksack_group_identification() {
    let test_cases = vec![
        ("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg", 18),
        ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw", 52)
    ];

    for (input, output) in test_cases {
        assert_eq!(rucksack_group_identification(input), output);
    }
}

const ASCII_UPPERCASE_START_INDEX: i32 = 65;
const ASCII_UPPERCASE_END_INDEX: i32 = 90;

const ASCII_LOWERCASE_START_INDEX: i32 = 96;
const ASCII_LOWERCASE_END_INDEX: i32 = 122;

/// Uppercase letters are worth more
const UPPERCASE_PRIORITY_BUFFER: i32 = 27;

fn determine_byte_value(item: &u8) -> Option<i32> {
    let item = *item as i32;
    if item >= ASCII_UPPERCASE_START_INDEX && item <= ASCII_UPPERCASE_END_INDEX {
        // Uppercase
        return Some(item - ASCII_UPPERCASE_START_INDEX + UPPERCASE_PRIORITY_BUFFER);
    } else if item >= ASCII_LOWERCASE_START_INDEX && item <= ASCII_LOWERCASE_END_INDEX {
        // Lowercase
        return Some(item - ASCII_LOWERCASE_START_INDEX);
    }

    None
}