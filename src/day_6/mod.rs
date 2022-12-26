use std::collections::HashSet;

use aoc_setup;

pub fn solve()
{
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 6, "\n");
    assert!(puzzle_data.len() == 1);
    println!("Part 1: {}", part_one(&puzzle_data[0]));
    println!("Part 2: {}", part_two(&puzzle_data[0]));
}

fn process(puzzle_input: &String, num_unique_chars: usize) -> i32
{
    let chars = puzzle_input.chars();
    let mut idx = 0;
    while idx < puzzle_input.len()
    {
        let buf_iter = chars.clone().skip(idx).take(num_unique_chars);
        let buffer: Vec<char> = buf_iter.clone().collect();
        let checker: HashSet<char> = HashSet::from_iter(buf_iter);
        if checker.len() == num_unique_chars
        {
            return (idx + num_unique_chars) as i32;
        }
        idx += 1;
    }
    return 0;
}

fn part_one(puzzle_input: &String) -> i32
{
    return process(puzzle_input, 4);
}

fn part_two(puzzle_input: &String) -> i32
{

    return process(puzzle_input, 14);
}
