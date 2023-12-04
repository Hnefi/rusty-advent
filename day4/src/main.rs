use std::collections::{HashSet, HashMap};
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

// a struct representing a card, that has its string, number of matches, and copies.
struct Card {
    _card_str: String,
    num_matches: u32,
    num_copies: u32
}

impl Card {
}

fn card_id(input_string: &String) -> u32 {
    let re = Regex::new(r"Card (\d+):").unwrap();
    let Some(capture) = re.captures(&input_string) else {
        println!("No card id could be captured!!");
        quit::with_code(1);
    };
    (&capture[1]).parse().unwrap()
}

fn matches_for_card(input_string: &String) -> u32 {
    // parse this card
    let card_split: Vec<&str> = input_string.split('|').collect();
    assert_eq!(card_split.len(), 2, "Expected a card could be split into two, actually split into {}.", card_split.len());

    let mut winning_numbers: HashSet<i32> = HashSet::new();
    let regex_spaced_digits = Regex::new(r"(\d+)\s").unwrap();
    regex_spaced_digits.captures_iter(card_split.first().unwrap()).map(|c | c.extract()).for_each(|(_matching_pattern, [n])| {
        //println!("Matched full pattern={}, digit={}",_matching_pattern, n);
        winning_numbers.insert(n.parse().unwrap());
    });

    let regex_second_part = Regex::new(r"\s+(\d+)").unwrap();
    let num_matches: u32 = regex_second_part.captures_iter(card_split.last().unwrap()).map(|c| c.extract()).map(
        |(_, [n])| {
            let held_number: i32 = n.parse().unwrap();
            match winning_numbers.contains(&held_number) {
                true => 1,
                false => 0
            }
        }
    ).sum();
    num_matches
}

// Parse a line of text and return the number of points this card is worth.
fn points_for_card(input_string: String) -> i32 {
    // println!("Processing card {}", input_string);
    // split the line on the pipe character, and then parse both halves using a regex to match
    // all numbers separated by spaces. Put the winning numbers into a hash table (fast lookup)
    // and then for all of the numbers we hold, count the number of hits in the hash table.
    // Number of points equals 2^(matches-1)
    let num_matches = matches_for_card(&input_string);
    match matches_for_card(&input_string) {
        0 => 0,
        _ => 2_i32.checked_pow(num_matches-1).unwrap()
    }
}

fn parse_cards(file_lines: &Vec<String>) -> (HashMap<u32, Card>, Vec<u32>) {
    let mut hmap: HashMap<u32, Card> = HashMap::new();
    let mut cards: Vec<u32> = Vec::new();
    let _ = file_lines.iter().map(|line| {
        let cid = card_id(line);
        cards.push(cid);
        hmap.insert(cid, Card { card_str: line.to_string(), num_matches: matches_for_card(line), num_copies: 1})
    }).count(); // consume with count just to run the map
    (hmap, cards)
}

fn eval_card(cid: u32, card_map: &mut HashMap<u32, Card>) {
    println!("Evaluating card {}", cid);
    let cur_card: &Card = card_map.get(&cid).unwrap();
    // base case
    if cur_card.num_matches == 0 {
        return
    }

    // generate vector of cards to increment copy count and then evaluate
    for inc in 1..=cur_card.num_matches {
        println!("Incrementing card {}", cid+inc);
        let c: &mut Card = card_map.get_mut(&(cid + inc)).unwrap();
        //println!("card map before {}", c.num_copies);
        c.num_copies += 1;
        //let d: &mut Card = card_map.get_mut(&(cid + inc)).unwrap();
        //println!("card map after {}", d.num_copies);
        eval_card(cid+inc, card_map);
    }
}

fn count_total_cards(dat: (HashMap<u32, Card>, Vec<u32>)) -> u32 {
    let mut card_map = dat.0;
    let mut card_index = dat.1;
    card_index.sort();

    // algorithm:
    // - go over the card index in increasing numerical order. Eval() all cards.
    // - eval() recursively evaluates all card copies immediately
    for cid in card_index.iter() {
        eval_card(*cid, &mut card_map);
    }

    // once done, sum up all card counts.
    let result = card_map.values().map(|card| {
        return card.num_copies;
    }).sum();
    return result;
}

fn main() {
    let file_lines = get_file_lines(get_file_name());
    // part 1
    // let card_sum: i32 = file_lines.iter().map(|line| {
    //     points_for_card(line.to_string())
    // }).sum();
    // println!("Cards are worth = {}", card_sum);
    // part 2 - count number of cards
    let num_cards = count_total_cards(parse_cards(&file_lines));
    println!("Total cards = {}", num_cards);
}
