// Goal: Rank all the hands from 1 to N, 1 being the worst, and N the best.
// Find all the "5 of a kinds", then "4 of a kinds", "full houses", "3 of a kinds", "2 pair", "1 pair", "high card"
// - Assign them ordinal numbers by putting them into a heap and popping them out sequentially
// - only thing to implement is a custom ">" operator to compare 2 hands.

use std::{collections::{HashMap, BinaryHeap}, env};

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

#[derive(Debug)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

struct CamelCardHand {
    cards: Vec<u8>,
    bid: i32,
    hand_type: HandType,
}

fn get_hand_type(hand: &str) -> HandType {
    // five of a kind, easy to check for -> all elements in vector are the same.
    let mut counts = HashMap::<char, u8>::new();
    hand.chars().for_each(|c| {
        counts
            .entry(c)
            .and_modify(|count| *count = *count + 1)
            .or_insert(1);
        //println!("Added {} to counts, new count {}", c, counts[&c]);
    });

    if counts.len() == 1 {
        //println!("Is five of a kind!");
        return HandType::FiveKind;
    }

    if counts.len() == 2 {
        // full house or 4 of a kind
        let four_kind = counts.values().any(|val| *val == 4);
        if four_kind {
            //println!("Is four of a kind!");
            return HandType::FourKind;
        } else {
            //println!("Is full house!");
            return HandType::FullHouse;
        }
    } else if counts.len() == 3 {
        // trips or 2 pair
        let trips = counts.values().any(|val| *val == 3);
        if trips {
            //println!("Is three of a kind!");
            return HandType::ThreeKind;
        } else {
            //println!("Is two pair!");
            return HandType::TwoPair;
        }
    } else if counts.len() == 4 {
        //println!("Is one pair!");
        return HandType::Pair;
    }
    // if nothing else, this is a high card hand
    assert_eq!(counts.len(), 5);
    HandType::High
}

fn build_hand(raw_string: &str) -> CamelCardHand {
    // there has to be a better way to optimize this instead of creating a
    // new hashmap every time
    let to_ordinal = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 8),
        ('8', 9),
        ('9', 10),
        ('T', 11),
        ('J', 12),
        ('Q', 13),
        ('K', 14),
        ('A', 15),
    ]);
    let cards_and_bid: Vec<&str> = raw_string.split_whitespace().collect();
    CamelCardHand {
        cards: cards_and_bid
            .first()
            .unwrap()
            .chars()
            .into_iter()
            .map(|c| to_ordinal[&c])
            .collect(),
        bid: cards_and_bid.last().unwrap().parse().unwrap(),
        hand_type: get_hand_type(cards_and_bid.first().unwrap()),
    }
}

fn main() {
    let fname = get_file_name();
    let hands: Vec<CamelCardHand> = std::fs::read_to_string(fname)
        .unwrap()
        .split('\n')
        .map(|s| build_hand(s))
        .collect();



    // make the heap from all the hands, and then push hands off it one-by-one
    // let heap = BinaryHeap::<CamelCardHand>::new();
    // hands.into_iter().for_each(|h| heap.push(h));
}
