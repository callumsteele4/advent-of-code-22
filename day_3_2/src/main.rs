use itertools::Itertools;
use std::fs;

fn find_duplicate_items(rucksack_one: &str, rucksack_two: &str) -> String {
    let mut duplicate_items = String::from("");
    for rucksack_one_item in rucksack_one.chars() {
        let duplicate_item = rucksack_two
            .chars()
            .find(|item| *item == rucksack_one_item)
            .unwrap_or_default();
        if duplicate_item != '\0' {
            duplicate_items.push(duplicate_item);
        }
    }
    duplicate_items
}

fn get_priority_for_item(item: char) -> i32 {
    match item {
        item if item.is_ascii_lowercase() => item as i32 - 'a' as i32 + 1,
        item if item.is_ascii_uppercase() => item as i32 - 'A' as i32 + 27,
        _ => 0,
    }
}

fn main() {
    let rucksacks =
        fs::read_to_string("test_file.txt").expect("Should have been able to read the file");

    let mut badge_item_priority_sum = 0;
    for (rucksack_one, rucksack_two, rucksack_three) in rucksacks.split("\n").tuples() {
        let duplicate_items = find_duplicate_items(rucksack_one, rucksack_two);
        let badge_item = find_duplicate_items(&duplicate_items, rucksack_three)
            .pop()
            .unwrap_or_default();
        badge_item_priority_sum += get_priority_for_item(badge_item);
    }

    println!("{}", badge_item_priority_sum)
}
