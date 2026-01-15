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
    // safe w/ dial [0, 99], so % 100
    // puzzle input consists of direction (R,L)
    // and distance (#), i.e. R19 moves to 19, then L9 moves to 10
    // dial starts at 50
    // the answer is the number of times the dial will point to `0` after a given sequence
    let lines1: Vec<String> = read_lines("input.txt");
    part1(lines1);
    // .split_off
    let lines2: Vec<String> = read_lines("input.txt");
    part2(lines2);
}

fn part2(lines: Vec<String>) {
    let mut dial = 50;
    let mut count = 0;

    for mut l in lines {
        let shift: i32; // initialize, but don't set
        let step: i32;
        if l.starts_with('R') {
            step = 1;
            shift = l.split_off(1).parse::<i32>().unwrap();
        } else if l.starts_with('L') {
            step = -1;
            shift = -l.split_off(1).parse::<i32>().unwrap();
        } else {
            panic!("Invalid input")
        }
        count = count + (shift.abs() / 100); // add the full rotations
        //println!("dial: {}, count: {}, shift: {}, step: {}", dial, count, shift, step);
        for _ in 0..(shift.abs()%100) { // loop for remaining values
            dial = dial + step;
            if dial % 100 == 0 {
                count = count + 1;
            }
        }
        while dial >= 100 {
            dial = dial - 100;
        }
        while dial < 0 {
            dial = dial + 100;
        }

    }

    println!("part2 count: {}, {}", count, dial); // 
}

fn part1(lines: Vec<String>) {
    let mut dial: i32 = 50;
    let mut count = 0;
    //println!("{}",dial);
    for mut l in lines {
        //print!("{} ",l);
        if l.starts_with('R') {
            dial = (dial + l.split_off(1).parse::<i32>().unwrap()) % 100;
        } else if l.starts_with('L') {
            dial = (dial - l.split_off(1).parse::<i32>().unwrap()) % 100;
        } else {
            panic!("Invalid input")
        }
        if dial == 0 {
            count = count + 1;
        }
        //println!(":{}", dial);
    }
    println!("part1 count: {}", count);
    // 1172
}
