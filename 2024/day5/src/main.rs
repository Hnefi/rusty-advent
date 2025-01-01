use std::collections::HashSet;

use commons::io_utilities::read_file_lines;

#[derive(PartialEq, Eq, Hash, Debug)]
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

fn build_page_updates(update_file: &String) -> Vec<Vec<u32>> {
    let lines = read_file_lines(update_file);
    lines
        .iter()
        .map(|line| {
            line.split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|element| element.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn build_reversed_orderings(page_updates: &[u32]) -> Vec<PageOrdering> {
    let mut v = Vec::new();
    for i in 0..page_updates.len() {
        for j in i + 1..page_updates.len() {
            v.push(PageOrdering {
                first: *page_updates.get(j).unwrap(),
                second: *page_updates.get(i).unwrap(),
            });
        }
    }
    // println!(
    //     "Starting from page_updates: {:?}, built reversed orderings: {:?}",
    //     page_updates, v
    // );
    v
}

/* Given a vec of updates, return a new vec with only the legal ones (i.e., those
* which obey all of the ordering rules)
*/
fn get_legal_updates(updates: &[Vec<u32>], rules: HashSet<PageOrdering>) -> Vec<Vec<u32>> {
    let mut legal_updates = Vec::new();
    updates.iter().for_each(|pages: &Vec<u32>| {
        // Algorithm: for each update, build all pairs of numbers that correspond to orderings
        // e.g., for an update 75,47,61,53,29, generate pairs:
        // -> (75, 47), (75, 61), (75, 53), (75, 29)
        // -> (47, 61), (47, 53), (47, 29)
        // -> (61, 53), (61, 29)
        // Then determine if all of these update orderings are legal by looking up the INVERSE
        // ordering in the ruleset. e.g., If the ordering (75,47) comes from the actual update
        // list, we look up (47,75). If that rule is present, then the current set of page updates
        // is illegal and we can reject it.
        // If all inverse orders are not prohibited, then the set of page updates is legal and we
        // can accept it.
        let reversed_orders = build_reversed_orderings(pages);
        if !reversed_orders
            .iter()
            .any(|page_order| rules.contains(page_order))
        {
            legal_updates.push(pages.clone());
        }
    });
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
    let legal_updates =
        get_legal_updates(&build_page_updates(&page_outputs), build_ruleset(&rules));
    //println!("Legal updates are: {:?}", legal_updates);

    let middle_value_sum: u32 = legal_updates
        .iter()
        .map(|page_updates| {
            // The middle index is equal to len() / 2 in integer division.
            page_updates.get(page_updates.len() / 2).unwrap()
        })
        .sum();
    println!("Final sum of middle-page numbers: {}", middle_value_sum);
}
