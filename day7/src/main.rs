// Goal: Rank all the hands from 1 to N, 1 being the worst, and N the best.
// Find all the "5 of a kinds", then "4 of a kinds", "full houses", "3 of a kinds", "2 pair", "1 pair", "high card"
// - Assign them ordinal numbers by sorting them based on the problem statement and popping them out sequentially
// - only thing to implement is a custom ">" operator to compare 2 hands.

use std::{collections::HashMap, env, cmp::Ordering};

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

// partial eq and eq can be done just with the native builtins, because
// they do compare every element of the vector
#[derive(PartialEq, Eq, Debug)]
struct CamelCardHand {
    hand_type: HandType,
    cards: Vec<u8>,
    bid: i32
}

impl PartialOrd for CamelCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_order = self.hand_type.partial_cmp(&other.hand_type);
        if type_order == None {
            return None;
        }
        if type_order.unwrap() == Ordering::Equal {
            return self.cards.partial_cmp(&other.cards);
        } else {
            return type_order;
        }
    }
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);
        if type_order == Ordering::Equal {
            return self.cards.cmp(&other.cards);
        } else {
            return type_order;
        }
    }
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
        ('J', 1), // part 2 - J is the weakest individual card
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
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
    let mut hands: Vec<CamelCardHand> = std::fs::read_to_string(fname)
        .unwrap()
        .split('\n')
        .map(|s| build_hand(s))
        .collect();
    hands.sort();

    let pts: i32 = hands
        .iter()
        .enumerate()
        .map(|h| (h.0+1) as i32 * h.1.bid)
        .sum();
    println!("Final points: {:}", pts);
}
