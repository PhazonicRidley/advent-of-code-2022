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
    let mut rope: Rope = Rope::new(10);

    for (direction, value) in input {
        // println!(
        //     "Head is at: {:?}, Tail is at: {:?}",
        //     rope.head_coord, rope.tail_coord
        // );
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

fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return 0;
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

    fn update_tail(&mut self, axis: usize, mut previous_head: [i32; 2]) {
        let stopping_point = self.knots.len() - 1;
        for idx in 0..stopping_point{
            let next_idx = idx + 1;
            let segment_distance = self.get_distance_head_tail(idx, next_idx);
            if segment_distance == f32::sqrt(5.0) {
                self.knots[next_idx] = previous_head;
            } else if segment_distance >= 2.0 {
                let offset = self.knots[idx][axis] - self.knots[next_idx][axis];
                let direction = if offset.is_negative() { -1 } else { 1 };
                self.knots[next_idx][axis] += direction;
            }

            previous_head = self.knots[next_idx].clone();
        }
        self.tail_locations.insert(self.knots[self.knots.len() - 1].clone());
    //    // println!("Tail distance from head: {}", tail_distance);
    //     if tail_distance == f32::sqrt(5.0) {
    //        self.tail_coord = previous_head; // previous axis position of head before it was incremented 
    //     }
    //     else if tail_distance >= 2.0 {
    //         let offset = self.head_coord[axis] - self.tail_coord[axis];
    //         let direction = if offset.is_negative() { -1 } else { 1 };
    //         self.tail_coord[axis] += direction;
    //     }
    //     self.tail_locations.insert(self.tail_coord.clone());
    }

    fn move_rope(&mut self, displacement: i32, axis: usize) {
       
        let direction = if displacement.is_negative() { -1 } else { 1 };
        for _ in 0..displacement.abs() {
            let previous_head = self.knots[0].clone();
            self.knots[0][axis] += direction;
            self.update_tail(axis, previous_head);
        }
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


