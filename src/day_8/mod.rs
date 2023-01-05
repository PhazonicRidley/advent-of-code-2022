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
    let tree = vector[tree_index];
    let mut results = (true, true);

    // prior to index:
    for i in (0..tree_index).rev() {
        if tree <= vector[i] {
            results.0 = false;
        }
    }

    for j in tree_index + 1..vector.len() {
        if tree <= vector[j] {
            results.1 = false;
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

fn fetch_visable_trees(puzzle_input: &Vec<String>) -> (i32, Vec<(usize, usize)>) {
    let trees = make_tree_matrix(puzzle_input);

    // To count the number of trees along the edges, we take the perimeter then subtract 4 to account for the double count of the corners.
    let mut number_seen = ((2 * trees[0].len()) + (2 * trees.len()) - 4) as i32;

    let mut counted_coordinate: Vec<(usize, usize)> = vec![];

    for i in 1..trees.len() - 1 {
        let row = &trees[i];
        for j in 1..row.len() - 1 {
            if is_visable(j, row) {
                number_seen += 1;
                counted_coordinate.push((i, j));
            }
        }
    }

    for i in 1..trees.len() - 1 {
        let col = get_column(&trees, i);
        for k in 1..col.len() - 1 {
            if is_visable(k, &col) && !counted_coordinate.contains(&(k, i)) {
                number_seen += 1;
            }
        }
    }

    return (number_seen, counted_coordinate);
}
fn part_one(puzzle_input: &Vec<String>) -> i32 {
    let result = fetch_visable_trees(puzzle_input);

    return result.0;
}

fn calculate_scenic_score(tree_index: usize, vector: &Vec<i32>) -> u64 {
    let tree = vector[tree_index];
    let mut prior_score = 0;
    let mut latter_score = 0;

    // prior to index:
    for i in (0..tree_index).rev() {
        prior_score += 1;
        if tree <= vector[i] {
            break;
        }
    }

    if prior_score == 0 {
        prior_score = 1;
    }
    for j in tree_index + 1..vector.len() {
        latter_score += 1;
        if tree <= vector[j] {
            break;
        }
    }

    if latter_score == 0 {
        latter_score = 1;
    }

    return prior_score * latter_score;
}

fn part_two(puzzle_input: &Vec<String>) -> u64 {
    let trees: Vec<Vec<i32>> = make_tree_matrix(puzzle_input);
    let mut highest_final_score = 0;
    for i in 1..trees.len() - 1 {

        let row = &trees[i];
        for j in 1..row.len() - 1 {
            let mut column_score: u64 = 1;
            let mut row_score: u64 = 1;
            row_score *= calculate_scenic_score(j, row);
            let column = get_column(&trees, j);
            column_score *= calculate_scenic_score(i, &column);
            let final_score = row_score * column_score;
            if final_score > highest_final_score {
                highest_final_score = final_score;
            }
        }  
    }

    return highest_final_score;
}
