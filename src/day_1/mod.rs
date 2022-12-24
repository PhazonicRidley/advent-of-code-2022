use aoc_setup;

pub fn solve()
{
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 1, "\n\n");
    println!("Part 1: {}", part_one(&puzzle_data)[0]);
    println!("Part 2: {}", part_two(&puzzle_data));
}

pub fn part_one(puzzle_data: &Vec<String>) -> Vec<i32>
{
    let mut sums: Vec<i32> = vec![];
    for inventory in puzzle_data
    {
        //println!("{:?}\n\n", inventory.split("\n").collect::<Vec<&str>>());
        let sum: i32 = inventory.split("\n").map(|s| s.parse::<i32>().unwrap_or(0)).sum();
        sums.push(sum);
    }
    //return sums;
    sums.sort();
    sums.reverse();
    return sums;
}

pub fn part_two(puzzle_data: &Vec<String>) -> i32
{
    let sums = part_one(puzzle_data);
    return sums[0..2].iter().sum();
}