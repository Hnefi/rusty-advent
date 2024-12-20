use regex::Regex;
use std::env;

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

fn match_mul_operands(input: String) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|captures| {
            let (_, [op1, op2]) = captures.extract();
            (op1.parse::<u32>().unwrap(), op2.parse::<u32>().unwrap())
        })
        .collect()
}

fn main() {
    let src_file = get_file_name();
    println!(
        r#"Hello from AOC 2024 Day 3, this time in astro!"
        "Running with problem input = {}"#,
        src_file
    );

    let input = std::fs::read_to_string(src_file).unwrap();
    println!("Parsing operands....");
    let parsed_operands = match_mul_operands(input);
    // println!(
    //     r#"Printing all the parsed operands = {:?}"#,
    //     parsed_operands
    // );
    let total: u32 = parsed_operands.into_iter().map(|(x, y)| x * y).sum();
    println!("Total sum of uncorrupted muls: {}", total);
}
