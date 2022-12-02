use std::env;
use advent_of_code;

pub fn main()
{
    let cookie = env::var("AOC_TOKEN").unwrap_or(String::new());
    let res = advent_of_code::read_and_parse_input_data(2022, 1, cookie.as_str(), "\n")
        .unwrap();
    part_one(res.clone());
    part_two(res);
}

fn part_one(puzzle_data: Vec<String>)
{
    println!("Test part one code");
}

fn part_two(puzzle_data: Vec<String>)
{
    println!("Test part two code");
}