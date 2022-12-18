use std::{fs, env, process::exit};
mod day_1;

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
    let cookie = fs::read_to_string("../token.txt").unwrap_or(String::from("err"));
    let res = advent_of_code::read_and_parse_input_data(2022, 1, cookie.as_str(), "\n\n");
    let data = match res {
        Ok(ok) => ok,
        Err(e) => panic!("The following error has happened: {}", e.to_string())
    };
    
    let p1 = day_1::part_one(&data);
    println!("{}", p1[0]);
    day_1::part_two(&data);
}
