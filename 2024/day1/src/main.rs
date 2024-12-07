use  std::{env, iter::zip};

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

fn get_lists_from_file(file_name: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec = Vec::<i32>::new();
    let mut right_vec = Vec::<i32>::new();
    let input = std::fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = input
        .split("\n")
        .collect();

    println!("Parsed file...");
    // Split lines by whitespace and put them into the two lists.
    lines.iter().for_each(|line| {
        let numbers: Vec<i32> = line.
            split_whitespace().
            map(|x| {let r = x.parse().ok().unwrap(); r}).
            collect();
        numbers.chunks(2).for_each(|slice| {
            left_vec.push(slice[0]);
            right_vec.push(slice[1]);
        });
    });
    println!("Sorting lists...");
    // Sort lists and then add up the differences.
    left_vec.sort();
    right_vec.sort();
    return (left_vec, right_vec);
}

fn main() {
    let fname = get_file_name();
    println!("Hello AOC 2024 Day 1!! Calculating list differences from file {}...", fname);
    let (left, right) = get_lists_from_file(fname);

    println!("Summing up...");
    let sum: u32 = zip(left, right)
        .map(|slice| {
            slice.0.abs_diff(slice.1)
        })
        .sum();
    println!("Sum of differences = {}", sum);
}