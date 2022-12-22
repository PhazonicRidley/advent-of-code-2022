use std::{fs, env, process::exit};

mod day_2;
mod day_3;

fn main() {
    // TODO: discover correct path
    // temporary solution: verify there is a src directory, and if not exit
    let mut src_path = env::current_dir().unwrap(); // this should always be Ok
    // TODO: see if there is a way to move current_dir
    src_path.push("src");
    if !src_path.exists()
    {
        println!("Please run from the cargo root directory.");
        exit(1);
    }
    let cookie_result = fs::read_to_string("token.txt");
    let cookie = match cookie_result {
        Ok(c) => c,
        Err(_) => panic!("Unable to read cookie, please check token.txt and location of that file.")
    };

    let res = aoc_setup::read_and_parse_input_data(2022, 3, cookie.as_str(), "\n");
    let data = match res {
        Ok(ok) => ok,
        Err(e) => panic!("The following error has happened: {}", e.to_string())
    };

    //main_day_2(&data);
    main_day_3(&data);
    
}

fn main_day_2(puzzle_input: &Vec<String>)
{
    day_2::part_one(puzzle_input);
    day_2::part_two(puzzle_input);
}

fn main_day_3(puzzle_input: &Vec<String>)
{
    println!("Part 1: {}", day_3::part_one(puzzle_input));
    println!("Part 2: {}",day_3::part_two(puzzle_input));
}