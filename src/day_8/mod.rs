use aoc_setup;

pub fn solve() {
    let puzzle_data = aoc_setup::get_puzzle_data(2022, 8, "\n");
    // println!("{:?}", );
    println!("Part 1: {}", part_one(&puzzle_data));
    println!("Part 2: {}", part_two(&puzzle_data));
}

fn make_tree_matrix(puzzle_input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut trees: Vec<Vec<i32>> = vec![];
    for line in puzzle_input {
        trees.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }

    return trees;
}

fn is_visable(tree_index: usize, vector: &Vec<i32>) -> bool {
    if vector.is_empty() {
        panic!("Empty vector");
    }
    let tree = vector[tree_index];
    let mut results = (true, true);

    // prior to index:
    for i in (0..tree_index).rev() {
        if tree <= vector[i] {
            results.0 = false;
           // println!("Tree of {} can't be seen over {} (index: {})", tree, vector[i], i);
        }
    }

    for j in tree_index + 1..vector.len() {
        if tree <= vector[j] {
            results.1 = false;
            //println!("Tree of {} can't be seen over {} (index: {})", tree, vector[j], j);
        }
    }


    return results.0 || results.1;
}




fn get_column(matrix: &Vec<Vec<i32>>, n: usize) -> Vec<i32> {
    return matrix
        .iter()
        .map(|v| v.iter().nth(n).unwrap())
        .map(|v| v.clone())
        .collect();
}

fn part_one(puzzle_input: &Vec<String>) -> i32 {
    let trees = make_tree_matrix(puzzle_input);

    // To count the number of trees along the edges, we take the perimeter then subtract 4 to account for the double count of the corners.
    let mut number_seen = ((2 * trees[0].len()) + (2 * trees.len()) - 4) as i32;
    
    let mut counted_coordinate: Vec<(usize, usize)> = vec![];

    for i in 1..trees.len() - 1 {
        let row = &trees[i];
        let mut check_columns = true;
        for j in 1..row.len() - 1 {
            if is_visable(j, row) {
                number_seen += 1;
                counted_coordinate.push((j, i));
            }
        }
    }

    for i in 1..trees.len() - 1 {
        let col = get_column(&trees, i);
        for k in 1..col.len() - 1 {
            if is_visable(k, &col) && !counted_coordinate.contains(&(i, k))
            {
                number_seen += 1;
            }
        }
    }


    return number_seen;
}

fn part_two(puzzle_input: &Vec<String>) -> i32 {
    return 0;
}
