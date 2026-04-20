use std::fs::read_to_string;
// rust readlines snippet from rust docs
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn parse_input(s: &str ) -> (Vec<char>, Vec<Vec<u32>>) {
    let mut v: Vec<String> = read_lines(s);
    // each entry in v is a single line

    let mut ops: Vec<String> = v.split_off(v.len()-1);// all except last line in `v`, last line in `ops`
    let res_ops: Vec<char> = ops[0].split(" ").filter_map(|x| x.parse::<char>().ok()).collect();
    //println!("{:?}", v.split_off(at));

    let mut inputs: Vec<Vec<u32>> = vec![];
    for i in v {
        // split on space and map to u32
        println!("{:?}", i[..i.len()-1].split(' ').collect::<Vec<&str>>());
        let split_line: Vec<u32> = i.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect();
        // each line is now a vector of u32 containing the nth input for all equations
        inputs.push(split_line);
    }

    // the last line of input is the series of operations performed
    //let ops = v[v.len() - 1].split(' ').map(String::from).collect();

    return (res_ops, inputs);
}

fn part1(s: &str) {
    let (ops, inputs) = parse_input(s);
    let mut result = 0;
    let inputs_per_op = inputs.len();
    for (index, c) in ops.iter().enumerate() {
        let mut op_res: u64 = 0;
        for i in 0..inputs_per_op {
            if i == 0 && *c == '*' {
                op_res = 1; // so we aren't * by 0
            }

            if *c == '+' {
                print!("+ {} ",inputs[i][index]);
                op_res += inputs[i][index] as u64;
            } else if *c == '*' {
                op_res *= inputs[i][index] as u64;
                print!("* {} ",inputs[i][index]);
            }
        }
        println!(" = {}", op_res);
        result += op_res;
    }

    println!("part1: {}", result);

}

fn main() {
    part1("input.txt");
}
