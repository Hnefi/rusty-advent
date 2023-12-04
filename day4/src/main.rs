use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use regex::Regex;

// Read argument for file input string.
fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len-1].clone()
}

// Open file and return a vector of all its lines.
fn get_file_lines(file_name: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file_string = read_to_string(file_name);
    for line in file_string.unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

// Parse a line of text and return the number of points this card is worth.
fn points_for_card(input_string: String) -> i32 {
    // println!("Processing card {}", input_string);
    // let re = Regex::new(r"Card (\d+):").unwrap();
    // let Some(capture) = re.captures(&input_string) else {
    //     println!("No card id could be captured!!");
    //     quit::with_code(1);
    // };
    // let card_id: i32 = (&capture[1]).parse().unwrap();

    // split the line on the pipe character, and then parse both halves using a regex to match
    // all numbers separated by spaces. Put the winning numbers into a hash table (fast lookup)
    // and then for all of the numbers we hold, count the number of hits in the hash table.
    // Number of points equals 2^(matches-1)
    let card_split: Vec<&str> = input_string.split('|').collect();
    assert_eq!(card_split.len(), 2, "Expected a card could be split into two, actually split into {}.", card_split.len());

    let mut winning_numbers: HashSet<i32> = HashSet::new();
    //let regex_spaced_digits = Regex::new(r"\s+(\d+)[\s+&&[^:]]*").unwrap();
    let regex_spaced_digits = Regex::new(r"(\d+)\s").unwrap();
    regex_spaced_digits.captures_iter(card_split.first().unwrap()).map(|c | c.extract()).for_each(|(_matching_pattern, [n])| {
        //println!("Matched full pattern={}, digit={}",_matching_pattern, n);
        winning_numbers.insert(n.parse().unwrap());
    });

    let regex_second_part = Regex::new(r"\s+(\d+)").unwrap();
    let num_matches: u32 = regex_second_part.captures_iter(card_split.last().unwrap()).map(|c| c.extract()).map(
        |(_, [n])| {
            let held_number: i32 = n.parse().unwrap();
            //println!("Checking hashset to see if it contained {}", held_number);
            match winning_numbers.contains(&held_number) {
                true => 1,
                false => 0
            }
        }
    ).sum();
    match num_matches {
        0 => 0,
        _ => 2_i32.checked_pow(num_matches-1).unwrap()
    }
}

fn main() {
    let file_lines = get_file_lines(get_file_name());
    let card_sum: i32 = file_lines.iter().map(|line| {
        points_for_card(line.to_string())
    }).sum();
    println!("Cards are worth = {}", card_sum);
}
