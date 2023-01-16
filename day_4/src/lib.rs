pub fn process_raw_line(line: &str) -> Vec<Vec<i32>> {
    line.split(",")
        .map(|s| {
            s.split("-")
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_contained(one_min: &i32, one_max: &i32, two_min: &i32, two_max: &i32) -> bool {
    let mut contains: bool = false;
    if one_min <= two_min && one_max >= two_max {
        contains = true;
    } else if two_min <= one_min && two_max >= one_max {
        contains = true;
    }
    contains
}

pub fn has_overlap(one_min: &i32, one_max: &i32, two_min: &i32, two_max: &i32) -> bool {
    let one_min_is_over_two = one_min >= two_min && one_min <= two_max;
    let one_max_is_over_two = one_max >= two_max && one_max <= two_min;
    let two_min_is_over_one = two_min >= one_min && two_min <= one_max;
    let two_max_is_over_one = two_max >= one_max && two_max <= one_min;

    one_min_is_over_two || one_max_is_over_two || two_min_is_over_one || two_max_is_over_one
}