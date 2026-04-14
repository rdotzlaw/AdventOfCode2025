use std::{cell::RefCell, cmp::min, fs::read_to_string};

// rust readlines snippet from rust docs
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}



fn main() {

    //part1();
    part2();
}

fn part2() {
    let input: Vec<String> = read_lines("input.txt").clone();
    let debug = false;
    let mut result:u64 = 0;
    let line_len = input[0].len();
    for mut line in input {
        let mut max: Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0,0,0];
        let mut index: usize = 0;
        let mut indicies: Vec<usize> = vec![0,0,0,0,0,0,0,0,0,0,0,0];
        // same as previous question, but this time its 12 instead of 2 digits
        let mut line_arr = line.into_bytes(); // unsafe { Vec::<u8>::from(line.as_bytes_mut()) }; // not actually unsafe, all ascii, so chars are u8 single byte
        line_arr.iter_mut().for_each(|i| *i = *i - 48); // from ascii code -> digit
        // line_arr is now an array of bytes where each element is equal to the digit in the input string
        // println!("{:?}", line_arr);
        
        let mut ind = 0;
        for i in 0..12 {
            let remaining = 12 - i;
            // last valid index to pick from
            let end = line_len - remaining;
            // find max digit and index in line_arr[pos..=end]
            let mut val = 0;
            let mut index = ind;
            for j in ind..=end {
                if line_arr[j] > val {
                    val = line_arr[j]; 
                    index = j;
                }
            }
            max[i] = val;
            ind = index + 1;
        }

        // add to result
        if debug {
            println!("--> max: {:?}\n--> ind: {:?}", max, indicies);
            println!("add to result");
        }
        let mut max_value: u64 = 0;
        let base: u64 = 10;
        for i  in 0..12 {
            max_value = max_value + ((max[i] as u64) * base.pow(12-(i as u32)-1));
            //println!("{}: {}\n--> {}",i, max[i], max_value);
        }
        result = result + max_value as u64;
        // break;

    }
    println!("part2: {}", result)
}
        
        /* 
        for i in 0..line_len {
            let current = line_arr[i]; 
            // iteratively check each `max` element
            for j in 0..max.len() {
                println!("{} --- current: {}, max[{}]: {}, max.len: {}, line_len: {}", i, current, j, max[j], max.len(), line_len);
                println!("{:?}", max);
                println!("---> {}: current > max[{}]\n---> {}: bounds", current > max[j], j, (max.len() - j) < (line_len - i - j));
                if current > max[j] && (max.len() - j) < (line_len - i - j) { // potentially wrong second condition
                    // cuts max[0..12] into two slices: left and _right
                    let (left, mut _right) = max.split_at_mut(j);
                    // from there, assign line_arr[j..(12 - j)] to the end of left and use that as new `max`
                    let conc: Vec<u8> = [left, &line_arr[j..(12 - j)]].concat();
                    max = conc.clone();
                    println!("concat: {:?}, len: {}", conc, conc.len());
                    //max[j..] = line_arr[j..(12 - (j + 1))];
                    //left.copy_from_slice(&line_arr[j..(12 - (j+i))]); 
                    // break out of loop, we don't want to assign current to a lower digit as it will make
                    // a smaller joltage than what we just made
                    break;

                }
            }
            break;*/

fn part1() {
    let input: Vec<String> = read_lines("input.txt").clone();
    let mut result:u64 = 0;
    let line_len = input[0].len();
    for line in input {
        // need to append two digits in line to create the largest possible number
        // cannot rearange the digits
        // i.e. line = "8769", line_number = 89
        // add up all line_number to get result

        // enumerate gives (index, value) on each .next() call on the iterator
        //println!("{}",line);
        let line_arr = line.as_bytes();
        //println!("{:?}", line_arr);
        // loop through line
        let mut max = 0;
        let mut max_ind = 0;
        let mut max2 = 0;
        let mut max2_ind = 0;
        for i in 0..line_len {
            let current = line_arr[i] - 48;
            if current > max && i != (line_len-1) {
                // if the current value is greater than our 10's digit `max`
                // and there are still values after it
                max = current; // primitive types, so shouldn't be affected if `current` mutates
                max_ind = i;
                max2 = line_arr[i+1] - 48;
                max2_ind = i+1;
            } else if current > max2 {
                max2 = current;
                max2_ind = i;
            }

            assert!(max2_ind >= max_ind, "Second index is before first index");
            // construct result
            //println!("{}{}", max, max2);
            //println!("-> {}",(10*max + max2)as u64);
            
        }
        result = result + (10*(max) + max2) as u64;
        
        //break;
    }
    
    println!("part1: {}", result);
}