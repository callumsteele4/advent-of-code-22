use std::fs;

fn main() {
    let elves =
        fs::read_to_string("test_file.txt").expect("Should have been able to read the file");

    let mut elf_calories = vec![0];

    for elf in elves.split("\n") {
        if elf == "" {
            elf_calories.push(0);
        } else {
            *elf_calories.last_mut().unwrap() += elf.parse::<i32>().unwrap();
        }
    }

    println!("{}", *elf_calories.iter().max().unwrap());
}
