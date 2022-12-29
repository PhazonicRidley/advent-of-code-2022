use std::collections::HashMap;

use aoc_setup;

pub fn solve() {
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 7, "\n");
    println!("Part 1: {}", part_one_but_good(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn folder_content_size(contents: &Vec<String>) -> i32 {
    let mut sums: Vec<i32> = vec![];
    for entry in contents {
        let entry_split: Vec<&str> = entry.split(" ").collect();
        if let Ok(size) = entry_split[0].parse::<i32>() {
            sums.push(size);
        }
    }

    return sums.iter().sum();
}

fn part_one(puzzle_input: &Vec<String>) -> i32 {
    let mut command_indices: Vec<usize> = vec![];
    for i in 0..puzzle_input.len() {
        if puzzle_input[i].chars().nth(0).unwrap() == '$' {
            command_indices.push(i);
        }
    }
    command_indices.reverse();
    let mut current_directory_path: Vec<&str> = vec![];
    let mut dir_sums: HashMap<String, i32> = HashMap::new();
    let mut command_idx = 0;
    let mut cd_str = String::new();
    let max_cmd_index = command_indices[0];

    while command_idx <= max_cmd_index  {
        let line_split: Vec<&str> = puzzle_input[command_idx].split(" ").collect();
        if line_split[1] == "cd" {
            if line_split[2] != ".." {
                current_directory_path.push(line_split[2]);
                cd_str = current_directory_path.join("/").to_string();
                dir_sums.insert(cd_str.clone(), 0);
            } else {
                let mut total_dir_sum = dir_sums.get(&cd_str).unwrap().to_owned();
                let sub_dirs: HashMap<String, i32> = dir_sums
                    .drain_filter(|k, _v| k.contains(&(cd_str.clone() + "/")))
                    .collect();

                total_dir_sum += sub_dirs.values().sum::<i32>();
                dir_sums.insert(cd_str.clone(), total_dir_sum);
                current_directory_path.remove(current_directory_path.len() - 1);
                cd_str = current_directory_path.join("/").to_string();
            }

            command_idx += 1;
        } else if line_split[1] == "ls" {
            let next_idx = command_indices.pop().unwrap_or(puzzle_input.len());
            let slice = puzzle_input[command_idx + 1..next_idx].to_vec();
            dir_sums.insert(cd_str.clone(), folder_content_size(&slice));
            command_idx = next_idx;
        }
        println!("current directory: {}", cd_str);
        println!("HashMap: {:?}", dir_sums);
        command_indices.drain_filter(|n| n.to_owned() <= command_idx);
    }

    dir_sums.insert(String::from("/"), dir_sums.values().sum());
    let final_sums: i32 = dir_sums.values().filter(|x| **x < 100000).sum();
    println!("Final Dir Sums: {:?}", dir_sums);
    return final_sums;
}

fn part_one_but_good(puzzle_input: &Vec<String>) -> i32
{
    let mut ls_indices: Vec<usize> = vec![];
    for i in 0..puzzle_input.len() {
        if puzzle_input[i] == "$ ls" {
            ls_indices.push(i);
        }
    }
    ls_indices.reverse();
    let mut command_idx = 0;

    let mut sums: Vec<i32> = vec![];
    let max_idx = ls_indices[0];
    while command_idx <= max_idx {
        let next_idx = ls_indices.pop().unwrap_or(puzzle_input.len());
        let dir_sum = folder_content_size(&puzzle_input[command_idx..next_idx].to_vec());
        if dir_sum < 100000 {
            sums.push(dir_sum);
        }
        command_idx = next_idx;
    }
    return sums.iter().sum();
}

fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return 0;
}
