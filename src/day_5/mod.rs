use std::collections::VecDeque;

use aoc_setup;



pub fn solve()
{
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 5, "\n");
    
    parse_input(&puzzle_data);
    println!("Part 1: {}", part_one(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn parse_input(puzzle_input: &Vec<String>) -> (Vec<VecDeque<char>>, Vec<String>)
{
    
    let crate_rows: Vec<String> = puzzle_input.iter().map(|s| s.to_owned()).take(8).collect();
    let instructions: Vec<String> = puzzle_input.iter().map(|s| s.to_owned()).skip(10).collect();
    let mut parsed_rows: Vec<Vec<char>> = vec![Vec::new(); 9];
    
    let mut row_idx = 0;
    for row in crate_rows
    {
        let chars = row.chars();
        let mut idx = 0;
        while idx < row.len()
        {
            let block: Vec<char> = chars.clone().skip(idx).take(4).collect();
            idx += 4;
            if block[0] == '[' && block[2] == ']'
            {
                parsed_rows[row_idx].push(block[1]);
            }
            else if block[1] == ' '
            {
                parsed_rows[row_idx].push(block[1]);
            }
        }
        row_idx += 1;
    }

    let mut column_stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    let mut col_idx = 0;
    for row in parsed_rows
    {
        for ch in row
        {
            if ch != ' '
            {
                column_stacks[col_idx].push_back(ch);
            }
            col_idx += 1;
        }
        col_idx = 0;
    }

   // println!("{:?}", column_stacks);
    return (column_stacks, instructions);
}

fn process_instruction(instruction: &String) -> (usize, usize, usize)
{
    let split:  Vec<&str> = instruction.split(" ").collect();
    let result_tuple = 
    (split[1].parse().unwrap(), split[3].parse::<usize>().unwrap() - 1, split[5].parse::<usize>().unwrap() - 1);

    return result_tuple;
}

fn move_crates(puzzle_input: &Vec<String>, keep_order: bool) -> String
{
    let (mut stacks, instructions) = parse_input(puzzle_input);
    for instruction in instructions
    {
        let (quantity, source, destination) = process_instruction(&instruction);
        let moved_data: &mut Vec<char> = &mut stacks[source].drain(0..quantity).collect();
        if keep_order
        {
            moved_data.reverse();
        }

        for item in moved_data
        {
            stacks[destination].push_front(item.to_owned());
        } 
    }

    return stacks.iter().map(|v| v[0]).collect();

    
}

fn part_one(puzzle_input: &Vec<String>) -> String
{
    return move_crates(puzzle_input, false);
}

fn part_two(puzzle_input: &Vec<String>) -> String
{
    return move_crates(puzzle_input, true);
}
