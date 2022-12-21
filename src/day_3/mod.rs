use std::{collections::HashSet};

fn get_priority(ch: char) -> i32
{
    if ch.is_uppercase()
    {
        return (ch as i32) - 38;
    }
    else {
        return (ch as i32) - 96;
    }
}
pub fn part_one(puzzle_input: &Vec<String>) -> i32
{
    let mut sums: Vec<i32> = vec![];
    for rutsack in puzzle_input
    {
        // parse string into two components
        let rut_vec: Vec<char> = rutsack.trim().chars().collect();
        let half_size = rut_vec.len() / 2;
        let first_component = rut_vec[0..half_size].to_vec();
       // print!("First Component: {:?}", first_component);
        let second_component = rut_vec[half_size..rut_vec.len()].to_vec();
        //println!(" || Second Component{:?}", second_component);
        assert!(first_component.len() == half_size && second_component.len() == half_size);
        // discover if there are duplicates (use set intersection)
        let duplicates: HashSet<char> = first_component.into_iter().collect::<HashSet<char>>().intersection(&(second_component.into_iter().collect())).map(|c| c.to_owned()).collect();
        // if there are duplicates, take third set and get their priorities
        sums.push(get_priority(duplicates.iter().next().unwrap().to_owned()));
    }
    // sum result
    return sums.iter().sum();
    
}

pub fn part_two(puzzle_input: &Vec<String>)
{
    // get groups
    // split items
}