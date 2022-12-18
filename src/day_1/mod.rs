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

pub fn part_two(puzzle_data: &Vec<String>)
{
    let sums = part_one(puzzle_data);
    println!("{:?}", sums[0..2].iter().sum::<i32>())
}