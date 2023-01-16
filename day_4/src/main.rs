use std::fs;

use day_4::{ process_raw_line, is_contained, has_overlap };

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