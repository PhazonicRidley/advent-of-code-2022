use std::collections::HashSet;

pub fn part_one(puzzle_input: &Vec<String>) -> i32 {
    let mut counter = 0;
    for elf_pair in puzzle_input {
        let mut elfs = elf_pair.split(",");
        let mut first_elf = elfs.next().unwrap().split("-");
        let mut second_elf = elfs.next().unwrap().split("-");
        let first_elf_range: Vec<i32> = first_elf.take(2).map(|i| i.parse().unwrap()).collect();
        let second_elf_range: Vec<i32> = second_elf.take(2).map(|i| i.parse().unwrap()).collect();
        if first_elf_range[0] <= second_elf_range[0] && first_elf_range[1] >= second_elf_range[1] {
            counter += 1
        } else if first_elf_range[0] >= second_elf_range[0]
            && first_elf_range[1] <= second_elf_range[1]
        {
            counter += 1
        }
    }

    return counter;
}

pub fn part_two(puzzle_input: &Vec<String>) -> i32 {
    let mut counter = 0;
    for elf_pair in puzzle_input {
        let mut elfs = elf_pair.split(",");
        let mut first_elf = elfs.next().unwrap().split("-");
        let mut second_elf = elfs.next().unwrap().split("-");
        let first_elf_range: HashSet<i32> = (first_elf.next().unwrap().parse::<i32>().unwrap()
            ..=first_elf.next().unwrap().parse::<i32>().unwrap())
            .collect();
        let second_elf_range: HashSet<i32> = (second_elf.next().unwrap().parse::<i32>().unwrap()
            ..=second_elf.next().unwrap().parse::<i32>().unwrap())
            .collect();

        if !first_elf_range.is_disjoint(&second_elf_range) {
            counter += 1;
        }
    }

    return counter;
}
