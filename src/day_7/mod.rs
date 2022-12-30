use std::collections::HashMap;

use aoc_setup;

pub fn solve() {
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 7, "\n");
    println!("Part 1: {}", part_one(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn folder_content_size(contents: &Vec<String>) -> i32 {
    let mut sums: Vec<i32> = vec![];
    let mut multiplier = 0;
    for entry in contents {
        let entry_split: Vec<&str> = entry.split(" ").collect();
        //println!("{:?}", entry_split);
        if let Ok(size) = entry_split[0].parse::<i32>() {
            sums.push(size);
        }
        // if entry.contains("..") {
        //     multiplier += 1;
        // }
    }

    
    println!("multiplier: {}", multiplier);

    return (sums.iter().sum::<i32>()); // * if multiplier != 0 { multiplier } else { 1 };
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
    let mut counted_dirs: Vec<String> = vec![];
    let mut command_idx = 0;
    let mut cd_str = String::new();
    let max_cmd_index = command_indices[0];

    while command_idx <= max_cmd_index {
        let line_split: Vec<&str> = puzzle_input[command_idx].split(" ").collect();
        if line_split[1] == "cd" {
            if line_split[2] != ".." {
                current_directory_path.push(line_split[2]);
                cd_str = current_directory_path.join("/").to_string();
                dir_sums.insert(cd_str.clone(), 0);
            } else {
                let mut total_dir_sum: i32 = dir_sums.get(&cd_str).unwrap().to_owned();
                
                for (dir_str, size) in &dir_sums {
                    if dir_str.contains(&(cd_str.clone() + "/")) && !counted_dirs.contains(dir_str) {
                        counted_dirs.push(dir_str.clone());
                        total_dir_sum += size;
                    }
                    
                }

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

        command_indices.drain_filter(|&mut n| n <= command_idx);
    }

    
    let mut counter = 0;
    let mut final_sum = 0;
    for value in dir_sums.values() {
        if value <= &100000 {
            counter += 1;
            final_sum += value;
        }
    }

    return final_sum;
}



fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return 0;
}
