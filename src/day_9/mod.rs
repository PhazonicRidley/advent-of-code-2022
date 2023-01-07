use std::{collections::HashSet, fmt::Display};

use aoc_setup;

pub fn solve() {
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 9, "\n");
    println!("Part 1: {}", part_one(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn parse_input(puzzle_input: &Vec<String>) -> Vec<(char, i32)> {
    let mut result: Vec<(char, i32)> = vec![];
    for line in puzzle_input {
        let mut split = line.split(" ");
        result.push((
            split.next().unwrap().chars().nth(0).unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    return result;
}

fn part_one(puzzle_input: &Vec<String>) -> i32 {
    let input = parse_input(puzzle_input);
    //println!("{:?}", input);
    let mut rope: Rope = Rope::new();

    return 0;
}

fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return 0;
}

#[derive(Debug)]
struct Rope {
    head_coord: (i32, i32),
    tail_coord: (i32, i32),
    tail_locations: HashSet<(i32, i32)>,
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Head at: ({}, {}), Tail at: ({}, {})",
            self.head_coord.0, self.head_coord.1, self.tail_coord.0, self.tail_coord.1
        )
    }
}

impl Rope {
    pub fn new() -> Self {
        return Rope {
            head_coord: (0, 0),
            tail_coord: (0, 0),
            tail_locations: HashSet::new(),
        };
    }

    // TODO: test this.
    fn update_tail(&mut self) {
        let displacement_coordinate = (
            self.head_coord.0 - self.tail_coord.0,
            self.head_coord.1 - self.tail_coord.1,
        );
        self.tail_coord.0 += displacement_coordinate.0;
        self.tail_coord.1 += displacement_coordinate.1;
    }

    fn verticle_movment(&mut self, displacement: i32) {
        self.head_coord.0 = displacement;
    }
}
