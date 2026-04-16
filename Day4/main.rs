use array2d::Array2D;
use std::fs::read_to_string;
// rust readlines snippet from rust docs
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn get_array2d(source_vec: Vec<String>) -> Array2D<char> {
    let mut result: Vec<Vec<char>> = vec![];
    for i in 0..source_vec.len() {
        let char_vec: Vec<char> = source_vec[i].chars().collect();
        result.push(char_vec);
    }
    return Array2D::from_rows(&result).unwrap();
}

fn count_neighbours(
    input: &Array2D<char>,
    row_index: usize,
    col_index: usize,
    row_count: usize,
    col_count: usize,
) -> bool {
    let mut count = 0;

    let above = row_index > 0;
    let below = row_index < row_count;
    let left = col_index > 0;
    let right = col_index < col_count;

    // can we check above?
    if above {
        // print!("{}", input[(row_index - 1, col_index)]);
        // direct above
        if input[(row_index - 1, col_index)] == '@' {
            count += 1;
        }
        // above left
        if left {
            if input[(row_index - 1, col_index - 1)] == '@' {
                count += 1;
            }
            // print!("{}", input[(row_index - 1, col_index - 1)]);
        }
        // above right
        if right {
            if input[(row_index - 1, col_index + 1)] == '@' {
                count += 1;
            }
            // print!("{}", input[(row_index - 1, col_index + 1)]);
        }
        // print!("\n");
    }

    // mid left
    if left {
        if input[(row_index, col_index - 1)] == '@' {
            count += 1;
        }
        // print!("{}", input[(row_index, col_index - 1)]);
    }
    // print!("{}", input[(row_index, col_index)]);
    // mid right
    if right {
        if input[(row_index, col_index + 1)] == '@' {
            count += 1;
        }
        // print!("{}", input[(row_index, col_index + 1)]);
    }
    // print!("\n");

    // can we check below?
    if below {
        // print!("{}", input[(row_index + 1, col_index)]);
        // direct below
        if input[(row_index + 1, col_index)] == '@' {
            count += 1;
        }
        // below left
        if left {
            if input[(row_index + 1, col_index - 1)] == '@' {
                count += 1;
            }
            // print!("{}", input[(row_index + 1, col_index - 1)]);
        }
        // below right
        if right {
            if input[(row_index + 1, col_index + 1)] == '@' {
                count += 1;
            }
            // print!("{}", input[(row_index + 1, col_index + 1)]);
        }
        // print!("\n");
    }

    //println!("count {}, at [{},{}], above: {}, below: {}, left: {}, right: {}", count, row_index, col_index, above, below, left, right);
    //println!("===============");

    return count < 4;
}

fn part1() {
    let input = get_array2d(read_lines("input.txt").clone());
    // can access a roll of paper `@` if there are < 4 rolls of paper (`@`) in the eight adjacent positions

    let mut result = 0;
    let line_len = input.num_columns() - 1;
    let row_count = input.num_rows() - 1;
    let _dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    println!("row: {}, col: {}", row_count, line_len);
    for i in 0..=row_count {
        //println!("{}", i);
        for j in 0..=line_len {
            //println!("({},{}): {}",i,j, input[(i,j)]);
            // input[(i,j)] is the current character

            // check all adjacent positions, keep record of instances of `@`
            // if count is >= 4, then you can't access it
            // otherwise, you can access it, add 1 to result

            if input[(i, j)] == '@' && count_neighbours(&input, i, j, row_count, line_len) {
                //print!("x");
                result += 1;
            } else if input[(i, j)] == '@' {
                //print!("@");
            } else {
                //print!(".");
            }
        }
        //print!("\n")
    }
    println!("part1: {}", result);
}

fn part2() {
    // we want to go through and remove paper from the collection
    // paper is removeable if `count_neighbours` returns true
    // keep iterating until no more paper can be removed
    // result is the total amount of paper that can be removed from the collection before stopping
    
    let mut input = get_array2d(read_lines("input.txt").clone());
    let mut result = 0;
    let line_len = input.num_columns() - 1;
    let row_count = input.num_rows() - 1;
    while true {
        // have we found paper to remove this loop?
        let mut this_loop = false;
        // loop thru 2d array to find the indicies of paper rolls to remove
        let mut indicies: Vec<(usize, usize)> = vec![];

        for i in 0..=row_count {
            for j in 0..=line_len {
                if input[(i, j)] == '@' && count_neighbours(&input, i, j, row_count, line_len) {
                    //print!("x");
                    indicies.push((i,j));
                    this_loop = true;
                    result += 1;
                }
            }
        }

        for ind in indicies {
            input[ind] = '.'; // remove all paper rolls that we can
        }

        // no more paper rolls to remove
        if !this_loop {
            break;
        }
    }
    println!("part2: {}", result);
}

fn main() {
    part1();
    part2();
}
