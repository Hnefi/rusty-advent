use std::{cmp::Ordering, collections::HashSet};

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
    v
}

fn build_all_orderings(page_updates: &[u32]) -> Vec<PageOrdering> {
    let mut v = Vec::new();
    for i in 0..page_updates.len() {
        for j in i + 1..page_updates.len() {
            v.push(PageOrdering {
                first: *page_updates.get(i).unwrap(),
                second: *page_updates.get(j).unwrap(),
            });
        }
    }
    v
}

fn swap_page_orderings(update: &mut [u32], swaps: &[PageOrdering]) {
    swaps.iter().for_each(|ordering| {
        // We unwrap these index finds because the slice of swaps has already been built from
        // the same 'update' vector, and if the elements are not found any longer, this is
        // a bug.
        let i1 = update
            .iter()
            .position(|item| item.cmp(&ordering.first) == Ordering::Equal)
            .unwrap();
        let i2 = update
            .iter()
            .position(|item| item.cmp(&ordering.second) == Ordering::Equal)
            .unwrap();
        if i1 > i2 {
            //println!("Performing swap for Page Ordering: {:?}", ordering);
            // Don't swap indices that are already in sorted order. This can happen
            // when multiple swaps are generated, and a previous swap has moved the
            // first element before the second.
            update.swap(i1, i2);
            //println!("Update vec after swap {:?}", update);
        }
    });
}

fn fix_illegal_update(update: &mut [u32], rules: &HashSet<PageOrdering>) {
    // Algorithm: In an infinite loop, fixup the rules list as follows:
    // - Generate all ordering pairs that break the provided rules.
    // - Swap all those two pairs.
    // - Run until there are no more swaps/orders to be fixed up.
    loop {
        //println!("Loop-fixing illegal update vec: {:?}", update);
        let reversed_orders = build_reversed_orderings(update);
        let illegal_orders_to_swap: Vec<PageOrdering> = reversed_orders
            .into_iter()
            .filter_map(|page_ordering| {
                if rules.contains(&page_ordering) {
                    Some(page_ordering)
                } else {
                    None
                }
            })
            .collect();
        // println!(
        //     "Orders violated in the illegal update: {:?}",
        //     illegal_orders_to_swap
        // );
        if illegal_orders_to_swap.is_empty() {
            break;
        } else {
            swap_page_orderings(update, &illegal_orders_to_swap);
        }
    }

    // Assert the vector now obeys all the rules! More specifically, assert that all
    // pages are strictly ordered before all their successors. If any page is not
    // strictly ordered before ALL its successors, then the new vector is not
    // guaranteed to be ordered.
    let page_orders = build_all_orderings(update);
    assert!(page_orders
        .iter()
        .all(|page_order| rules.contains(page_order)));
}

#[derive(Debug, Eq, PartialEq)]
enum UpdateLegality {
    Legal,
    Illegal,
}

/* Given a vec of updates, return a new vec with only updates matching the provided legality.
* For legal updates, we return those which obey all of the ordering rules.
* For illegal updates, we return those which do not.
*/
fn get_update_subset(
    updates: &[Vec<u32>],
    rules: &HashSet<PageOrdering>,
    legality: UpdateLegality,
) -> Vec<Vec<u32>> {
    let mut filtered_updates = Vec::new();
    if legality == UpdateLegality::Legal {
        updates.iter().for_each(|pages: &Vec<u32>| {
            // Algorithm: for each update, build all pairs of numbers that correspond to orderings
            // e.g., for an update 75,47,61,53,29, generate pairs:
            // -> (75, 47), (75, 61), (75, 53), (75, 29)
            // -> (47, 61), (47, 53), (47, 29)
            // -> (61, 53), (61, 29)
            // Then determine if all of these update orderings are legal by looking up the
            // ordering in the ruleset.
            let page_orders = build_all_orderings(pages);
            if page_orders
                .iter()
                .all(|page_order| rules.contains(page_order))
            {
                filtered_updates.push(pages.clone());
            }
        })
    } else {
        assert!(legality == UpdateLegality::Illegal);
        updates.iter().for_each(|pages: &Vec<u32>| {
            let reversed_orders = build_reversed_orderings(pages);
            if reversed_orders
                .iter()
                .any(|page_order| rules.contains(page_order))
            {
                filtered_updates.push(pages.clone());
            }
        })
    }
    filtered_updates
}

fn get_middle_page_number_sum(updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .map(|page_updates| {
            // assert no even-numbered update vecs, breaks the middle-index calc
            assert_eq!(page_updates.len() % 2, 1);
            // The middle index is equal to len() / 2 in integer division.
            page_updates.get(page_updates.len() / 2).unwrap()
        })
        .sum()
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
    let page_updates = build_page_updates(&page_outputs);
    let legal_updates = get_update_subset(&page_updates, &rules, UpdateLegality::Legal);
    println!(
        "Part 1: Sum of middle-page numbers: {}",
        get_middle_page_number_sum(&legal_updates)
    );
    let mut illegal_update_list = get_update_subset(&page_updates, &rules, UpdateLegality::Illegal);

    // Sanity check that the update is broken down into two lists as expected.
    assert_eq!(
        legal_updates.len() + illegal_update_list.len(),
        page_updates.len(),
    );
    illegal_update_list
        .iter_mut()
        .for_each(|update| fix_illegal_update(update, &rules));
    println!(
        "Part 2: Sum of middle-page numbers: {}",
        get_middle_page_number_sum(&illegal_update_list)
    );
}
