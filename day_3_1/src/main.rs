use std::fs;

fn find_duplicate_item(compartment_one: &str, compartment_two: &str) -> char {
    let mut duplicate_item = '\0';
    for compartment_one_item in compartment_one.chars() {
        duplicate_item = compartment_two
            .chars()
            .find(|item| *item == compartment_one_item)
            .unwrap_or_default();
        if duplicate_item != '\0' {
            break;
        }
    }
    duplicate_item
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

    let mut duplicate_item_priority_sum = 0;
    for rucksack in rucksacks.split("\n") {
        let (compartment_one, compartment_two) = rucksack.split_at(rucksack.len() / 2);
        let duplicate_item = find_duplicate_item(compartment_one, compartment_two);
        duplicate_item_priority_sum += get_priority_for_item(duplicate_item);
    }

    println!("{}", duplicate_item_priority_sum)
}
