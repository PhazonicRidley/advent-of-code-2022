use std::collections::HashMap;

fn prep_map<'a>() -> HashMap<&'a str, i32>
{
    let mut dict: HashMap<&str, i32> = HashMap::new();
    dict.insert("A", 1);
    dict.insert("B", 2);
    dict.insert("C", 3);
    dict.insert("X", 1);
    dict.insert("Y", 2);
    dict.insert("Z", 3);
    return dict;
}
pub fn part_one(puzzle_data: &Vec<String>)
{
    let dict = prep_map();

    let mut sums: Vec<i32> = vec![];
    for turn in puzzle_data
    {
        print!("{}:  ", turn);
        let mut turn_split = turn.split(" ");
        let opposing_move = turn_split.next().unwrap();
        let my_move = turn_split.next().unwrap();
        assert!(["A", "B", "C"].contains(&opposing_move));
        assert!(["X", "Y", "Z"] .contains(&my_move));
        //let mut win_value = 0; // defaults to loss value, no additional points
        print!("{} + ", dict[my_move]);
        if my_move == "Z" && opposing_move == "A"
        {
            sums.push(3 + 0);
        }
        else if my_move == "X" && opposing_move == "C"
        {
            sums.push(1 + 6);
        }
        else if dict[my_move] > dict[opposing_move]
        {
            sums.push(dict[my_move] + 6); 
            print!("{} ", 6);
        }
        else if dict[my_move] < dict[opposing_move]
        {
            sums.push(dict[my_move] + 0);
            print!("{} ", 0);
        }
        else if dict[my_move] == dict[opposing_move] // the game is a tie
        {
            sums.push(dict[my_move] + 3); 
            print!("{} ", 3);
        }
        println!("= {}", sums[sums.len() - 1]);
    }
    assert!(puzzle_data.len() == sums.len());
    let total: i32 = sums.iter().sum();
    //println!("{:?}", sums);
    println!("{}", total);
    // determine what im playing
    // determine what opponent is playing
    // outcome of game?
    // sum everything
}

pub fn part_two(puzzle_data: &Vec<String>)
{
    let mut sums: Vec<i32> = vec![];
    let dict = prep_map();
    for turn in puzzle_data
    {
        let mut turn_split = turn.split(" ");
        let opposing_move = turn_split.next().unwrap();
        let mode = turn_split.next().unwrap();
        let mut my_move: i32 = match mode {
            "X" => (dict[opposing_move] - 1) % 3,
            "Y" => dict[opposing_move],
            "Z" => (dict[opposing_move] + 1) % 3,
            _ => panic!("This should never be seen")
        };
        if my_move == 0
        {
            my_move = 3;
        }

        let win_value: i32 = match mode {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("This message should not appear.")
        };
       
        println!("\nMy move: {}", my_move);
        println!("{}: {} + {}", turn, my_move, win_value);
        sums.push(my_move + win_value);
    }
    assert!(sums.len() == puzzle_data.len());
    let total: i32 = sums.iter().sum();
    println!("{}", total);

}