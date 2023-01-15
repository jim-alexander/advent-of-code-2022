use std::fs;

fn main() {
    let elf_calorie_count = fs::read_to_string("./input.txt").unwrap();

    let mut elfs = group_elf_food_by_elf(&elf_calorie_count);
    let (largest, calories) = get_largest(&elfs).unwrap();
    println!("The elf with the most food is: {} with: {} calories.", largest, calories);

    elfs = sort_descending(elfs);
    let top_three = sum_top_three(&elfs);
    println!("The top three elvs are carrying: {}", top_three);
}

fn group_elf_food_by_elf(list: &str) -> Vec<i32> {
    let mut elfs = Vec::new();

    let mut sum = 0;
    for line in list.lines() {
        if line == "" {
            elfs.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    elfs.sort();
    elfs.reverse();

    return elfs;
}

fn sort_descending(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list.reverse();
    list
}

fn get_largest(list: &Vec<i32>) -> Option<(usize, &i32)> {
    list.iter()
        .enumerate()
        .max_by_key(|&(_i, x)| x)
        .map(|(i, x)| (i + 1, x))
}
fn sum_top_three(list: &Vec<i32>) -> i32 {
    &list[0] + &list[1] + &list[2]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_group_elfs() {
        let raw_input = "\
2494
8013
1055

5425
9104
10665

10642
10677
10300

"; // Figured out writing this test that the last group is never calculated unless it has a blank line at the end!

        assert_eq!(group_elf_food_by_elf(&raw_input), vec![31619, 25194, 11562]);
    }

    #[test]
    fn should_sort_descending() {
        let unsorted = vec![3, 5, 1, 100, 20];

        assert_eq!(sort_descending(unsorted), vec![100, 20, 5, 3, 1]);
    }

    #[test]
    fn should_get_largest() {
        let items = vec![100, 200, 65, 1, 4];

        assert_eq!(get_largest(&items), Some((2, &200)));
    }
}