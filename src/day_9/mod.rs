use std::{collections::HashSet, fmt::Display};

use aoc_setup;

pub fn solve() {
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 9, "\n");
    println!("Part 1: {}", part_one(&puzzle_data));
    //println!("Part 2: {}", part_two(&puzzle_data));
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

fn do_problem(puzzle_input: &Vec<String>, number_of_knots: usize) -> i32 {
    let input = parse_input(puzzle_input);
    let mut rope: Rope = Rope::new(number_of_knots);

    for (direction, value) in input {
        println!("{} {}", direction, value);
        match direction {
            'U' => rope.up(value),
            'R' => rope.right(value),
            'D' => rope.down(value),
            'L' => rope.left(value),
            _ => panic!("Invalid movment"),
        }
    }

    println!("Recorded tail locations: {:?}", rope.tail_locations);
    let res = rope.tail_locations.len();
    return res as i32;
}

fn part_one(puzzle_input: &Vec<String>) -> i32 {
    return do_problem(puzzle_input, 2);
}

fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return do_problem(puzzle_input, 10);
}

#[derive(Debug)]
struct Rope {
    knots: Vec<[i32; 2]>,
    tail_locations: HashSet<[i32; 2]>,
}

impl Rope {
    pub fn new(number_of_knots: usize) -> Self {
        let origin = [0, 0];
        let mut rope = Rope {
            knots: vec![origin; number_of_knots], // head is 0, tail is last item in vector
            tail_locations: HashSet::new(),
        };
        rope.tail_locations.insert(origin);
        return rope;
    }

    // TODO: test this.
    fn get_distance_head_tail(&self, start_idx: usize, end_idx: usize) -> f32 {
        let x_coord = (self.knots[start_idx][0] - self.knots[end_idx][0]).pow(2);
        let y_coord = (self.knots[start_idx][1] - self.knots[end_idx][1]).pow(2);
        let result = f32::sqrt((x_coord + y_coord) as f32);
        return result;
    }

    fn update_tail(&mut self, axis: usize) {
        for idx in 0..self.knots.len() - 1 {

            let next_idx = idx + 1;
            
            let segment_distance = self.get_distance_head_tail(idx, next_idx);
            let x_offset = self.knots[idx][0] - self.knots[next_idx][0];
            let y_offset = self.knots[idx][1] - self.knots[next_idx][1];
            //println!("x offset {}, y offset: {}, segment distance {}", x_offset, y_offset, segment_distance);
            if segment_distance == f32::sqrt(5.0) || segment_distance == f32::sqrt(8.0) {
                let x_direction = if x_offset.is_negative() { -1 } else { 1 };
                let y_direction = if y_offset.is_negative() { -1 } else { 1 };

                self.knots[next_idx][0] += x_direction; // TODO: move up/down diagonally instead of using previous
                self.knots[next_idx][1] += y_direction; // TODO: change it so direction is calculated differently for each axis 
            } else if segment_distance >= 2.0 {
                let offset = self.knots[idx][axis] - self.knots[next_idx][axis];
                let direction = if offset.is_negative() { -1 } else { 1 };
                self.knots[next_idx][axis] += direction;
            }

        }
        self.tail_locations
            .insert(self.knots[self.knots.len() - 1].clone());
    }

    fn move_rope(&mut self, displacement: i32, axis: usize) {
        let direction = if displacement.is_negative() { -1 } else { 1 };
        for i in 0..displacement.abs() {
            self.knots[0][axis] += direction;
            self.update_tail(axis);
            assert!(self.verify_tail(), "Invalid tail found at {} out of a displacement of {}", i, displacement);
        }
    }

    fn verify_tail(&self) -> bool {
        for idx in 0..self.knots.len() - 1 {
            let next_idx = idx + 1;
            let distance = self.get_distance_head_tail(idx, next_idx);
            if distance != 1.0 && distance != f32::sqrt(2.0) && distance != 0.0 {
                println!(
                    "Tail is invalid due to segment starting at {} (Point: {:?}) and ending at {} (Point: {:?}) Had a distance of {}\n, full tail: {:?}\n",
                    idx, self.knots[idx], next_idx,  self.knots[next_idx], distance, self.knots);
                return false;
            }

        }

        return true;
    }

    fn left(&mut self, displacement: i32) {
        self.move_rope(-displacement, 0);
    }

    fn right(&mut self, displacement: i32) {
        self.move_rope(displacement, 0);
    }

    fn up(&mut self, displacement: i32) {
        self.move_rope(displacement, 1);
    }

    fn down(&mut self, displacement: i32) {
        self.move_rope(-displacement, 1);
    }
}
