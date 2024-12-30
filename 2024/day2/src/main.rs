use std::{cmp::Ordering, env, str::FromStr};
fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

fn get_lists_from_file(file_name: String) -> Vec<String> {
    let input = std::fs::read_to_string(file_name).unwrap();
    let lines: Vec<String> = input
        .split("\n")
        .map(|f| String::from_str(f).ok().unwrap())
        .collect();
    lines
}

fn level_pair_is_safe(level_1: u32, level_2: u32) -> (i32, bool) {
    // return tuple:
    //  - element one: 1 if level_2 is > level_1, 0 if the two are equal, and -1 if level 2 < level_1
    //  - element 2: true if the difference is within the defined range [1,3]
    let diff = level_1.abs_diff(level_2);
    let diff_safe = matches!(diff, 1..=3);
    match level_2.cmp(&level_1) {
        Ordering::Greater => (1, diff_safe),
        Ordering::Equal => (0, diff_safe),
        Ordering::Less => (-1, diff_safe),
    }
}

fn is_report_safe(levels: &Vec<u32>) -> bool {
    //println!("Looking at levels {:?}", levels);

    let starting_value = -5;
    //let mut level_values_safe  = Vec::<bool>::new();
    let mut last_comparison: i32 = starting_value;

    for slice in levels.windows(2) {
        //println!("\tLooking at slice {:?}", slice);
        let (comparison, safe) = level_pair_is_safe(slice[0], slice[1]);
        //println!("\tcomparison = {}, safe={}", comparison, safe);
        //level_values_safe.push(safe);
        if last_comparison != starting_value && comparison != last_comparison {
            return false;
        }
        if !safe {
            return false;
        }
        last_comparison = comparison;
        //println!("last_comparison= {}", last_comparison);
    }
    true
}

fn get_level_vec_from_report(s: &str) -> Vec<u32> {
    let levels: Vec<u32> = s
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    levels
}

fn generate_dampened_reports(original_report: &str) -> Vec<Vec<u32>> {
    let mut dampened_reports = Vec::<Vec<u32>>::new();
    let levels = get_level_vec_from_report(original_report);

    //println!("Generating dampened reports from original report: {:?}", levels);
    for i in 0..levels.len() {
        let mut dampened_vec = levels.clone();
        dampened_vec.remove(i);
        //println!("Adding dampened report : {:?}", dampened_vec);
        dampened_reports.push(dampened_vec);
    }
    dampened_reports
}

fn main() {
    let fname = get_file_name();
    println!(
        "Hello AOC 2024 Day 2!! Calculating list safety from file {}...",
        fname
    );

    let reports = get_lists_from_file(fname);

    println!("Counting reports...");
    let safe_reports = reports
        .iter()
        .filter(|report| {
            let levels = get_level_vec_from_report(report);
            let base_safe = is_report_safe(&levels);
            if !base_safe {
                let dampened_reports: Vec<Vec<u32>> = generate_dampened_reports(report);
                return dampened_reports.iter().any(is_report_safe);
            }
            true
        })
        .count();
    println!("The number of safe reports is {}", safe_reports);
}
