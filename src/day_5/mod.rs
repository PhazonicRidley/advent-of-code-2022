use aoc_setup;



pub fn solve()
{
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 5, "\n");
    
    parse_input(&puzzle_data);
    println!("Part 1: {}", part_one(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn parse_input(puzzle_input: &Vec<String>) -> (Vec<Vec<char>>, Vec<String>)
{

    let crate_rows: Vec<String> = puzzle_input.iter().map(|s| s.to_owned()).take(8).collect();
    let instructions: Vec<String> = puzzle_input.iter().map(|s| s.to_owned()).skip(10).collect();
    let mut parsed_rows: Vec<Vec<char>> = vec![vec![]; 9];
    
    let mut row_idx = 0;
    for row in crate_rows
    {
        let mut chars = row.chars();
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

    let mut column_stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut col_idx = 0;
    for row in parsed_rows
    {
        for ch in row
        {
            column_stacks[col_idx].push(ch);
            col_idx += 1;
        }
        col_idx = 0;
    }

   // println!("{:?}", column_stacks);
    return (column_stacks, instructions);
}

fn part_one(puzzle_input: &Vec<String>) -> i32
{
    return 0;
}

fn part_two(puzzle_input: &Vec<String>) -> i32
{
    return 0;
}
