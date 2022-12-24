#![warn(dead_code)]

use aoc_setup;

mod day_2;
mod day_3;
mod day_1;
mod day_4;

fn main() {
    let puzzle_data = get_puzzle_data(2022, 4, "\n");
    println!("Part 1: {}", day_4::part_one(&puzzle_data));
    println!("Part 2: {}", day_4::part_two(&puzzle_data));
}

fn get_puzzle_data(year: i32, day: i32, split_over: &str) -> Vec<String>
{
    let res = match aoc_setup::read_and_parse_input_data(year, day, split_over) {
        Ok(data) => data,
        Err(e) => panic!("The following error has happened: {}", e.to_string())
    };

    return res
}

