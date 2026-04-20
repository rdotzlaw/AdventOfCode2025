use std::fs::read_to_string;
// rust readlines snippet from rust docs
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}


fn part1() {
    // given list of ranges (inclusive)
    // given list of ingredient IDs
    // count the # of ingredient IDs that fall in one of the given ranges
    let ranges_in = read_lines("input_1.txt").clone();
    let ids_in = read_lines("input_2.txt").clone();

    let mut result = 0;

    // parse ranges into numbers
    let mut ranges: Vec<(u64, u64)> = vec![];
    for r in ranges_in {
        let split: Vec<&str> = r.split('-').collect();
        let (lower, upper) = (split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap());
        ranges.push((lower, upper));
    }

    // parse ids into numbers
    let mut ids: Vec<u64> = vec![];
    for i in ids_in {
        ids.push(i.parse::<u64>().unwrap());
    }

    // pre-processing done

    // now check each ID to see if it falls in a range or not
    for i in ids {
        for (l,u) in &ranges {
            if i >= *l && i <= *u {
                result += 1;
                break;
            }
        }
    }

    println!("part1: {}", result);
}

fn part2() {
    // given list of ranges (inclusive)
    // count the total # of values that fall in all of the given ranges
    // beware of overlapping ranges, can't just `result += upper - lower`
    let ranges_in: Vec<String> = read_lines("input_1.txt").clone();

    let mut result = 0;

    // parse ranges into numbers
    let mut ranges: Vec<(u64,u64)> = vec![];

    for r in ranges_in {
        let split: Vec<&str> = r.split('-').collect();
        let (lower, upper) = (split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap());
        ranges.push((lower, upper));
    }

    // sort ranges
    ranges.sort_by(|(al, _au), (bl, _bu)| al.cmp(bl));

    let mut buckets: Vec<(u64,u64)> = vec![];

    // loop through all ranges
    for (mut lower, mut upper) in ranges.clone() {
        // loop through the buckets, if they exist
        if buckets.len() == 0 {
            buckets.push((lower, upper));
        } else {
            let mut index = 0;
            let mut add_flag = true;
            for (bucket_lower, bucket_upper) in buckets.clone() {
                let lower_in = lower >= bucket_lower && lower <= bucket_upper;
                let upper_in = upper >= bucket_lower && upper <= bucket_upper;
                if lower_in && upper_in {
                    // current range is entirely encompased by the current bucket
                    // don't add this to buckets
                    add_flag = false;
                    break;
                } else if lower_in && !upper_in {
                    // lower is the only part w/in the bucket's range
                    // adjust range
                    lower = bucket_upper + 1;
                } else if !lower_in && upper_in {
                    // upper is the only part w/in the bucket's range
                    // adjust range
                    upper = bucket_lower - 1;
                }

                if upper < lower || lower > upper {
                    // we've adjusted too much, this bucket is no good
                    add_flag = false;
                }
            }
            if add_flag {
                buckets.push((lower, upper));
            }
        }

    }

    // we now have a collection of buckets
    //println!("buckets: {:?}\n ranges: {:?}", buckets, ranges);
    
    for (l, u) in buckets {
        result += u - l + 1;
    }

    println!("part2: {}", result)
}


fn main() {
    part1();
    part2();
}
