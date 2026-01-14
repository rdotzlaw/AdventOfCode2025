use std::fs::read_to_string;

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
    let lines2: Vec<String> = read_lines("test.txt");
    part2(lines2);
}

fn part2(lines: Vec<String>) {
    let mut dial = 50;
    let mut count = 0;
    for mut l in lines {
        let mut shift = 0;
        let mut r_flag = true;
        if l.starts_with('R') {
            shift = l.split_off(1).parse::<i32>().unwrap();
        } else if l.starts_with('L') {
            shift = -l.split_off(1).parse::<i32>().unwrap();
            r_flag = false;
        }
        // determine how many times `dial` will pass 0 with `shift` applied to it

        // find `distance` to 0
        let full_distance = dial + shift;
        let distance = shift % 100;
        println!("dial: {}, count: {}", dial, count);
        println!("shift: {}, full: {}, dist: {}", shift, full_distance, distance);
        if (r_flag && full_distance > 99) || (!r_flag && full_distance <= 0) {
            // we'll cross 0 at least once when turning dial
            println!("crossed 0 ");
            count = count + ((full_distance - distance) / 100);
        }
        dial = (dial + shift) % 100;
        println!("");
    }
    println!("count: {}", count); // 5591468
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
    println!("count: {}", count);
    // 1172
}
