use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut sum_contained = 0;
    let mut sum_overlapped = 0;

    for line in input.lines() {
        let pair = process_raw_line(&line);

        let one_min = &pair[0][0];
        let one_max = &pair[0][1];

        let two_min = &pair[1][0];
        let two_max = &pair[1][1];

        if is_contained(&one_min, &one_max, &two_min, &two_max) {
            sum_contained += 1;
        }
        if has_overlap(&one_min, &one_max, &two_min, &two_max) {
            sum_overlapped += 1;
        }
    }

    println!("Sum contained: {sum_contained}");
    println!("Sum overlaped: {sum_overlapped}")
}

fn process_raw_line(line: &str) -> Vec<Vec<i32>> {
    line.split(",")
        .map(|s| {
            s.split("-")
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
#[test]
fn test_process_raw_line() {
    let lines = vec![
        ("2-4,6-8", vec![vec![2, 4], vec![6, 8]]),
        ("2-3,4-5", vec![vec![2, 3], vec![4, 5]]),
        ("5-7,7-9", vec![vec![5, 7], vec![7, 9]]),
        ("2-8,3-7", vec![vec![2, 8], vec![3, 7]]),
        ("6-6,4-6", vec![vec![6, 6], vec![4, 6]]),
        ("2-6,4-8", vec![vec![2, 6], vec![4, 8]])
    ];

    for (input, output) in lines {
        assert_eq!(process_raw_line(&input), output);
    }
}

fn is_contained(one_min: &i32, one_max: &i32, two_min: &i32, two_max: &i32) -> bool {
    let mut contains: bool = false;
    if one_min <= two_min && one_max >= two_max {
        contains = true;
    } else if two_min <= one_min && two_max >= one_max {
        contains = true;
    }
    contains
}
#[test]
fn test_is_contained_true() {
    assert!(is_contained(&1, &10, &2, &9));
    assert!(is_contained(&-5, &5, &-4, &4));
    assert!(is_contained(&-10, &10, &-9, &9));
}
#[test]
fn test_is_contained_false() {
    assert!(!is_contained(&1, &10, &11, &20));
    assert!(!is_contained(&-5, &5, &6, &10));
    assert!(!is_contained(&-10, &10, &-15, &-5));
}

fn has_overlap(one_min: &i32, one_max: &i32, two_min: &i32, two_max: &i32) -> bool {
    let one_min_is_over_two = one_min >= two_min && one_min <= two_max;
    let one_max_is_over_two = one_max >= two_max && one_max <= two_min;
    let two_min_is_over_one = two_min >= one_min && two_min <= one_max;
    let two_max_is_over_one = two_max >= one_max && two_max <= one_min;

    one_min_is_over_two || one_max_is_over_two || two_min_is_over_one || two_max_is_over_one
}
#[test]
fn test_has_overlap_true() {
    assert!(has_overlap(&1, &10, &5, &15));
    assert!(has_overlap(&-5, &5, &-2, &2));
    assert!(has_overlap(&-10, &10, &-5, &5));
    assert!(has_overlap(&1, &10, &9, &20));
    assert!(has_overlap(&-5, &5, &-10, &-2));
}
#[test]
fn test_has_overlap_false() {
    assert!(!has_overlap(&1, &10, &11, &20));
    assert!(!has_overlap(&-5, &5, &6, &10));
    assert!(!has_overlap(&-10, &10, &11, &20));
}