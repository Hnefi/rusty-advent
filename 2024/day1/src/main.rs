use std::iter::zip;

use commons::arg_parsing::get_file_name_or_quit;

fn get_lists_from_file(file_name: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec = Vec::<i32>::new();
    let mut right_vec = Vec::<i32>::new();
    let input = std::fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    println!("Parsed file...");
    // Split lines by whitespace and put them into the two lists.
    lines.iter().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().ok().unwrap())
            .collect();
        numbers.chunks(2).for_each(|slice| {
            left_vec.push(slice[0]);
            right_vec.push(slice[1]);
        });
    });
    println!("Sorting lists...");
    // Sort lists and then add up the differences.
    left_vec.sort();
    right_vec.sort();
    (left_vec, right_vec)
}

fn main() {
    let fname = get_file_name_or_quit();
    println!(
        "Hello AOC 2024 Day 1!! Calculating list differences from file {}...",
        fname
    );
    let (left, right) = get_lists_from_file(fname);

    println!("Summing up...");
    let sum: u32 = zip(left, right)
        .map(|slice| slice.0.abs_diff(slice.1))
        .sum();
    println!("Sum of differences = {}", sum);
}
