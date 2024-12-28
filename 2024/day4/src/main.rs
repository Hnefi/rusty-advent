use commons::arg_parsing::get_file_name_or_quit;

mod word_search;

fn main() {
    let fname = get_file_name_or_quit();
    println!("Hello from AOC Day 4! Parsing puzzle: {}", fname);

    let mut word_search = word_search::build_board_from_file(&fname);
    // for every starting index in the board, evaluate all potential matches passing through
    // this point (for both parts)
    for idx in 0..word_search.board.len() {
        let pot_matches =
            word_search::generate_potential_matches_part_one(idx.try_into().unwrap(), &word_search);
        word_search::evaluate_words(pot_matches, &mut word_search);

        if word_search.board[idx] == 'A' {
            word_search::evaluate_words_part_two(
                word_search::generate_potential_matches_part_two(
                    idx.try_into().unwrap(),
                    &word_search,
                ),
                &mut word_search,
            );
        }
    }

    // The number of matches is the size of the set "matched_sequences"
    println!(
        "The number of matches is: {}... And for part two: {}",
        word_search.matched_sequences_part_one.len(),
        word_search.matched_sequences_part_two.len(),
    );
    // println!(
    //     "Exact matches is: {:?}, and evaluated sequences: {:?}",
    //     word_search.matched_sequences, word_search.tested_sequences
    // )
}
