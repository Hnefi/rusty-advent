use std::collections::HashSet;

use commons::io_utilities::read_file_lines;

#[derive(PartialEq, Eq, Hash)]
struct PageOrdering {
    first: u32,
    second: u32,
}

fn build_ruleset(rule_file: &String) -> HashSet<PageOrdering> {
    let mut ret_set = HashSet::new();
    let rules = read_file_lines(rule_file);
    rules.iter().for_each(|rule| {
        let page_numbers: Vec<&str> = rule.split('|').collect();
        let before_page = page_numbers[0].parse::<u32>().unwrap();
        let after_page = page_numbers[1].parse::<u32>().unwrap();
        // println!(
        //     "Parsed page numbers: {:?}, and before: {}, and after: {}",
        //     page_numbers, before_page, after_page
        // );
        ret_set.insert(PageOrdering {
            first: before_page,
            second: after_page,
        });
    });
    ret_set
}

/* Given a vec of updates, return a new vec with only the legal ones (i.e., those
* which obey all of the ordering rules)
*/
fn get_legal_updates(updates: &Vec<Vec<u32>>, rules: HashSet<PageOrdering>) -> Vec<Vec<u32>> {
    let mut legal_updates = Vec::new();
    legal_updates
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len != 3 {
        println!("Incorrect number of provided args = {:?}", args);
        return;
    }
    let page_outputs = args[arg_len - 1].clone();
    let rules = args[arg_len - 2].clone();
    println!(
        "Hello from AOC Day 5! Parsing puzzle input rules: {rules}, and also parsing page outputs: {page_outputs}",
    );

    let rules = build_ruleset(&rules);
    let page_updates = read_file_lines(&page_outputs);
}
