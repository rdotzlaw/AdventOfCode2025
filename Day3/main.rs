use std::{fs::read_to_string};

// rust readlines snippet from rust docs
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}


fn main() {

}

fn part1() {
    let input: Vec<String> = read_lines("input.txt").clone();
    let mut result: u64 = 0;
    for line in input {
        // need to append two digits in line to create the largest possible number
        // cannot rearange the digits
        // i.e. line = "8769", line_number = 89
        // add up all line_number to get result
    }
}