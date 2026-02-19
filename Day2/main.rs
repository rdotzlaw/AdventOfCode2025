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
    part1();
    part2();
}

fn part2() {
        // only one line
    let input: String = read_lines("input.txt")[0].clone();
    // input is a series of ranges, i.e.: 100-900,2452-9084, ... etc.
    // look for invalid #'s in the given ranges
    // a number is invalid if is only made up of a sequence of digits repeated twice, i.e.:
    // 11 -> 1 repeated, 123123 -> 123 repeated, 1010 -> 10 repeated, etc.
    // no numbers have leading zeros, so 0101 isn't a number you'd need to check
    // --> find all invalid id's and add them up
    // println!("{}", input);
    let ranges: Vec<&str> = input.split(',').collect();
    let mut invalid: u64 = 0; // sum of all invalid ids
    for r in ranges {
        // println!("{}", r);
        let split: Vec<&str> = r.split('-').collect();
        let (lower, upper) = (split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap());
        let (_lower_len, _upper_len) = (split[0].len(), split[1].len());

        invalid = invalid + check_id2(lower, upper);

    }
    println!("final invalid: {}", invalid);
}

fn check_id2(lower:u64, upper:u64) -> u64 {
    // invalid id's are constructed by some repeating pattern
    // ie: 12341234: 1234 twice, 1111: 1 four times, etc.

    let mut invalid: u64 = 0;

    for i in lower..(upper +1 ) { // inclusive upper bounds
        let digits = i.checked_ilog10().unwrap_or(0) + 1; // get i's digits
        let mid = digits / 2; // if digits is odd, will give floor
        for x in 0..mid { // [0,mid)
            let i_copy = i.to_string();
            if digits % (mid-x) == 0 { // can be equally split
                // split i_copy into (mid-x) sized chunks
                let mut chunks: Vec<&str> = i_copy.as_bytes() // convert to byte vector
                    .chunks((mid-x) as usize) // split into chunks of size (mid-x)
                    .map(str::from_utf8) // map into a utf8 from bytes
                    .collect::<Result<Vec<&str>,_>>() // collect into a Result<Vec, utf8Error>
                    .unwrap(); // unwrap to get Vec
                // we have a vector of equal sized chunks
                // now, check if all are equal
                chunks.dedup();
                if chunks.len() == 1 { // we only have a single deduped element, so they're all the same
                    // therefore, this id is invalid
                    invalid = invalid + i;
                    break;
                }
                // iter will skip over elements until it reaches the end or closure returns false
                //if chunk_iter.next() == None {
                    // skipped all elements in iter, therefore all elements are equal
                //}
            }
        }
        
    }

    return invalid;
}

fn part1() {
    // only one line
    let input: String = read_lines("input.txt")[0].clone();
    // input is a series of ranges, i.e.: 100-900,2452-9084, ... etc.
    // look for invalid #'s in the given ranges
    // a number is invalid if is only made up of a sequence of digits repeated twice, i.e.:
    // 11 -> 1 repeated, 123123 -> 123 repeated, 1010 -> 10 repeated, etc.
    // no numbers have leading zeros, so 0101 isn't a number you'd need to check
    // --> find all invalid id's and add them up
    // println!("{}", input);
    let ranges: Vec<&str> = input.split(',').collect();
    let mut invalid: u64 = 0; // sum of all invalid ids
    for r in ranges {
        // println!("{}", r);
        let split: Vec<&str> = r.split('-').collect();
        let (lower, upper) = (split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap());
        let (_lower_len, _upper_len) = (split[0].len(), split[1].len());

        invalid = invalid + check_id(lower, upper);

    }
    println!("final invalid: {}", invalid);
}

fn check_id(lower:u64, upper:u64) -> u64 {
    let mut invalid = 0;

    for i in lower..(upper + 1) { // +1, b/c we want to check upper as well
        let i_digits = i.checked_ilog10().unwrap_or(0) + 1; // get i's digits
        if i_digits % 2 == 0 {
            // string is in utf-8, so we can't index it
            // convert to bytes, since string will only have ascii chars this can be done
            let i_str = i.to_string();
            let mid = i_digits/2;
            // println!("{}",i);
            if i_str[..mid as usize] == i_str[mid as usize..] {
                // then id is invalid
                invalid = invalid + i; // add up all the invalid ids
            }
            // stringify and split `i` in half and compare halves to each other
        } // if digits aren't even, pass (lower = 10, upper = 100000), need to account for odd digit count in search
        
    }

    return invalid;
}

// adjusts a given range value to a non-odd # of digits range
fn _rebuild_range(range: u64, is_lower: bool) -> u64 {
    let num_digits: u32 = range.checked_ilog10().unwrap_or(0) + 1;
    if num_digits % 2 != 0 { // odd number of digits in range
        if is_lower { // is lower bound, increase digits
            let mut res: u64 = 1;
            for _ in 0..num_digits {
                res = res * 10;
            }
            return res;
        } else { // is upper bound, decrease digits
            let mut res: u64 = 9;
            for _ in 0..(num_digits - 2){
                res = (res * 10) + 9;
            }
            return res;
        }
    } else {
        return range; // return previous range, since it's even # of digits
    }
}