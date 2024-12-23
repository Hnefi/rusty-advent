use commons::arg_parsing::get_file_name_or_quit;

mod word_search;

fn main() {
    let fname = get_file_name_or_quit();
    println!("Hello from AOC Day 4! Parsing puzzle: {}", fname);

    let mut word_search = word_search::build_board_from_file(&fname);
    // for every starting index in the board, evaluate all potential matches passing through
    // this point
    for idx in 0..word_search.board.len() {
        let pot_matches =
            word_search::generate_potential_matches(idx.try_into().unwrap(), &word_search);
        word_search::evaluate_words(pot_matches, &mut word_search);
    }

    // The number of matches is the size of the set "matched_sequences"
    println!(
        "The number of matches is: {}",
        word_search.matched_sequences.len()
    );
    // println!(
    //     "Exact matches is: {:?}, and evaluated sequences: {:?}",
    //     word_search.matched_sequences, word_search.tested_sequences
    // )
}
