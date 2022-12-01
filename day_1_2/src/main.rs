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

    elf_calories.sort();
    elf_calories.reverse();

    println!("{:?}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
