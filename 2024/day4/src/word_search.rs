//pub mod word_search {
use commons::io_utilities::read_file_lines;
use std::{cmp::Ordering, collections::HashSet, iter::zip};

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct IndexSequencePartOne(pub i32, pub i32, pub i32, pub i32);

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct IndexSequencePartTwo(pub i32, pub i32, pub i32);

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct XmasSequencePair(pub IndexSequencePartTwo, pub IndexSequencePartTwo);

pub trait InBounds {
    fn is_horizontal_in_bounds(&self, board: &WordSearchBoard) -> bool;
    fn is_vertical_in_bounds(&self, board: &WordSearchBoard) -> bool;
    fn is_diagonal_in_bounds(&self, board: &WordSearchBoard) -> bool;
}

impl IndexSequencePartOne {
    pub fn as_array(&self) -> [i32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl IndexSequencePartTwo {
    pub fn as_array(&self) -> [i32; 3] {
        [self.0, self.1, self.2]
    }
}

impl InBounds for XmasSequencePair {
    fn is_vertical_in_bounds(&self, board: &WordSearchBoard) -> bool {
        let seqs_out_of_bounds: Vec<bool> = [self.0, self.1]
            .iter()
            .map(|seq: &IndexSequencePartTwo| {
                seq.as_array()
                    .into_iter()
                    .any(|val| val < 0 || val >= board.board.len().try_into().unwrap())
            })
            .collect();
        seqs_out_of_bounds.into_iter().all(|out_bounds| !out_bounds)
    }

    fn is_horizontal_in_bounds(&self, board: &WordSearchBoard) -> bool {
        if self.is_vertical_in_bounds(board) {
            let seqs_in_bounds: Vec<bool> = [self.0, self.1]
                .iter()
                .map(|seq: &IndexSequencePartTwo| {
                    // Algorithm: calculate the "row number" for all the elements in the seq. If they
                    // are all identical, the horizontal sequence doesn't wrap and it's valid.
                    let rows: Vec<i32> = seq
                        .as_array()
                        .into_iter()
                        .map(|elem| elem / board.board_line_length)
                        .collect();
                    let first_row = rows.first().unwrap();
                    if rows.iter().all(|element| element == first_row) {
                        return true;
                    }
                    false
                })
                .collect();
            return seqs_in_bounds.into_iter().all(|in_bounds| in_bounds);
        }
        false
    }

    fn is_diagonal_in_bounds(&self, board: &WordSearchBoard) -> bool {
        // Since diagonal sequences are vertical, use the existing "vertical_in_bounds" fn.
        if self.is_vertical_in_bounds(board) {
            let seqs_in_bounds: Vec<bool> = [self.0, self.1]
                .iter()
                .map(|seq: &IndexSequencePartTwo| {
                    // Algorithm: Calculate the "column number" for all the elements in the seq.
                    // If the sequence is sorted, there is no wrapping around the horizontal boundary
                    let cols: Vec<i32> = seq
                        .as_array()
                        .into_iter()
                        .map(|elem| elem % board.board_line_length)
                        .collect();
                    //println!("Cols = {:?}", cols);
                    let cmp_greater_than = |a: &i32, b: &i32| {
                        if a.cmp(b) == Ordering::Greater {
                            return true;
                        }
                        false
                    };
                    if cols.is_sorted() || cols.is_sorted_by(cmp_greater_than) {
                        //println!("Cols list is sorted...");
                        return true;
                    }
                    false
                })
                .collect();
            return seqs_in_bounds.into_iter().all(|in_bounds| in_bounds);
        }
        false
    }
}

impl InBounds for IndexSequencePartOne {
    fn is_vertical_in_bounds(&self, board: &WordSearchBoard) -> bool {
        if self
            .as_array()
            .into_iter()
            .any(|val| val < 0 || val >= board.board.len().try_into().unwrap())
        {
            return false;
        }
        true
    }

    fn is_horizontal_in_bounds(&self, board: &WordSearchBoard) -> bool {
        if self.is_vertical_in_bounds(board) {
            // Algorithm: calculate the "row number" for all the elements in the seq. If they
            // are all identical, the horizontal sequence doesn't wrap and it's valid.
            let rows: Vec<i32> = self
                .as_array()
                .into_iter()
                .map(|elem| elem / board.board_line_length)
                .collect();
            let first_row = rows.first().unwrap();
            if rows.iter().all(|element| element == first_row) {
                return true;
            }
        }
        false
    }

    fn is_diagonal_in_bounds(&self, board: &WordSearchBoard) -> bool {
        // Since diagonal sequences are vertical, use the existing "vertical_in_bounds" fn.
        if self.is_vertical_in_bounds(board) {
            // Algorithm: Calculate the "column number" for all the elements in the seq.
            // If the sequence is sorted, there is no wrapping around the horizontal boundary
            let cols: Vec<i32> = self
                .as_array()
                .into_iter()
                .map(|elem| elem % board.board_line_length)
                .collect();
            //println!("Cols = {:?}", cols);
            let cmp_greater_than = |a: &i32, b: &i32| {
                if a.cmp(b) == Ordering::Greater {
                    return true;
                }
                false
            };
            if cols.is_sorted() || cols.is_sorted_by(cmp_greater_than) {
                //println!("Cols list is sorted...");
                return true;
            }
        }
        false
    }
}

// Uncomment for part one.
static WORD_TO_MATCH: &str = "XMAS";

pub struct WordSearchBoard {
    pub board: Vec<char>,
    pub board_line_length: i32, // the line length of the board, so we know when rows wrap
    // A HashMap which stores unique sequences of indices that correspond to a matching
    // key of XMAS. For two examples, consider this board.
    //      X M A S
    //      M B A X
    //      A M A S
    //      X C B Z
    // Starting at the board's top left, the horizontal match
    // XMAS would be represented as (0,1,2,3).
    // The match that starts from the bottom left and goes diagonally backwards
    // to the top right would be represented as (12,9,6,3).
    pub matched_sequences_part_one: HashSet<IndexSequencePartOne>,
    pub tested_sequences_part_one: HashSet<IndexSequencePartOne>,

    // Looking for 'MAS' strings in an X pattern, these variables store the sequence pairs
    // matched and tested for part two.
    pub matched_sequences_part_two: HashSet<XmasSequencePair>,
    pub tested_sequences_part_two: HashSet<XmasSequencePair>,
}

pub fn build_test_board() -> WordSearchBoard {
    let test_file = "test_input".to_string();
    build_board_from_file(&test_file)
}

pub fn build_board_from_file(fname: &String) -> WordSearchBoard {
    let rows = read_file_lines(fname);
    let board_line_length = rows[0].len();
    let mut board = Vec::new();
    rows.iter().for_each(|row| {
        let mut char_vec: Vec<char> = row.chars().collect();
        board.append(&mut char_vec);
    });
    WordSearchBoard {
        board,
        board_line_length: board_line_length as i32,
        matched_sequences_part_one: HashSet::new(),
        tested_sequences_part_one: HashSet::new(),
        matched_sequences_part_two: HashSet::new(),
        tested_sequences_part_two: HashSet::new(),
    }
}

pub fn generate_part_one_sequence(base_index: i32, delta: i32) -> IndexSequencePartOne {
    IndexSequencePartOne(
        base_index,
        base_index + delta,
        base_index + 2 * delta,
        base_index + 3 * delta,
    )
}

pub fn generate_part_two_sequence(base_index: i32, delta: i32) -> IndexSequencePartTwo {
    IndexSequencePartTwo(base_index, base_index + delta, base_index + 2 * delta)
}

pub fn add_new_part_one_sequence_if_untested(
    seq: IndexSequencePartOne,
    board: &WordSearchBoard,
    sequences: &mut HashSet<IndexSequencePartOne>,
) -> bool {
    if !board.tested_sequences_part_one.contains(&seq) {
        sequences.insert(seq);
        return true;
    }
    false
}

pub fn generate_potential_matches_part_two(
    base_index: i32,
    board: &WordSearchBoard,
) -> HashSet<XmasSequencePair> {
    // TODO
}

pub fn generate_potential_matches_part_one(
    base_index: i32,
    board: &WordSearchBoard,
) -> HashSet<IndexSequencePartOne> {
    // Generate a list of WordIndexes that represents all the possible
    // matches of the WORD_TO_MATCH that could overlap with the provided 'base_index'.

    let mut matches = HashSet::new();
    let word_len = WORD_TO_MATCH.len();
    //  - Neg offsets correspond to "forwards-reading" sequences where 'WORD_TO_MATCH' is read
    //  normally (meaning the word starts from a previous index to 'base_index')
    let negative_offsets = (-(word_len as i32) + 1)..=0;
    //  - Pos offsets correspond to "backwards-reading" sequences where 'WORD_TO_MATCH' is read
    //  in reverse character order (meaning the word starts from an index larger than 'base_index')
    let positive_offsets = 0..word_len as i32;
    negative_offsets.into_iter().for_each(|off| {
        // Type 1: vertical sequences.
        let start = base_index + (board.board_line_length * off);
        let seq: IndexSequencePartOne = generate_part_one_sequence(start, board.board_line_length);
        //println!("Generated seq {:?}", seq);
        if seq.is_vertical_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 2: horizontal sequences.
        let start = base_index + off;
        let seq: IndexSequencePartOne = generate_part_one_sequence(start, 1);
        //println!("Generated seq {:?}", seq);
        if seq.is_horizontal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 3: diagonal sequences.
        let start = base_index + (board.board_line_length * off) + off;
        let seq: IndexSequencePartOne =
            generate_part_one_sequence(start, board.board_line_length + 1);
        //println!("Generated seq {:?}", seq);
        if seq.is_diagonal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }
        let start = base_index + (board.board_line_length * off) - off;
        let seq: IndexSequencePartOne =
            generate_part_one_sequence(start, board.board_line_length - 1);
        //println!("Generated seq {:?}", seq);
        if seq.is_diagonal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }
    });

    positive_offsets.into_iter().for_each(|off| {
        // Type 1: vertical sequences.
        let start = base_index + (board.board_line_length * off);
        let seq: IndexSequencePartOne = generate_part_one_sequence(start, -board.board_line_length);
        //println!("Generated seq {:?}", seq);
        if seq.is_vertical_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 2: horizontal sequences.
        let start = base_index + off;
        let seq: IndexSequencePartOne = generate_part_one_sequence(start, -1);
        //println!("Generated seq {:?}", seq);
        if seq.is_horizontal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 3: diagonal sequences.
        let start = base_index + (board.board_line_length * off) + off;
        let seq: IndexSequencePartOne =
            generate_part_one_sequence(start, -board.board_line_length - 1);
        //println!("Generated seq {:?}", seq);
        if seq.is_diagonal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }
        let start = base_index + (board.board_line_length * off) - off;
        let seq: IndexSequencePartOne =
            generate_part_one_sequence(start, -board.board_line_length + 1);
        //println!("Generated seq {:?}", seq);
        if seq.is_diagonal_in_bounds(board) {
            //println!("Added {:?}", seq);
            add_new_part_one_sequence_if_untested(seq, board, &mut matches);
        }
    });
    matches
}

pub fn evaluate_seq_for_match(seq: &IndexSequencePartOne, board: &WordSearchBoard) -> bool {
    let v: Vec<char> = WORD_TO_MATCH.chars().collect();
    let z = zip(seq.as_array(), v);
    z.into_iter().all(|(seq_index, expected_char)| {
        if board.board[seq_index as usize] == expected_char {
            return true;
        }
        false
    })
}

pub fn evaluate_words(
    potential_matches: HashSet<IndexSequencePartOne>,
    board: &mut WordSearchBoard,
) {
    potential_matches.iter().for_each(|seq| {
        if evaluate_seq_for_match(seq, board) {
            // found an XMAS
            board.matched_sequences_part_one.insert(*seq);
        }
        board.tested_sequences_part_one.insert(*seq);
    });
}

#[cfg(test)]
mod tests {
    use super::*; // bring in all the functionality in the word_search module

    #[test]
    fn test_generate_sequence() {
        let base = 4;
        let test_positive_delta = 2;
        assert_eq!(
            generate_part_one_sequence(base, test_positive_delta),
            IndexSequencePartOne(4, 6, 8, 10)
        );
        let test_negative_delta = -3;
        assert_eq!(
            generate_part_one_sequence(base, test_negative_delta),
            IndexSequencePartOne(4, 1, -2, -5)
        );
    }

    #[test]
    fn test_add_new_sequence_if_untested() {
        let mut board = build_test_board();
        let new_seq = IndexSequencePartOne(1, 2, 3, 4);
        let mut sequences_to_test: HashSet<IndexSequencePartOne> = HashSet::new();
        // assert new sequence is added to sequences_to_test
        assert!(add_new_part_one_sequence_if_untested(
            new_seq,
            &board,
            &mut sequences_to_test
        ));
        // assert vec has the new_seq value
        assert!(sequences_to_test.contains(&new_seq));

        board.tested_sequences_part_one.insert(new_seq);
        // assert trying to add the same sequence now is false
        assert!(!add_new_part_one_sequence_if_untested(
            new_seq,
            &board,
            &mut sequences_to_test
        ));
    }

    #[test]
    fn test_is_vertical_sequence_in_bound() {
        let board = build_test_board();
        // test that a vertical sequence going off the board negatively is rejected
        assert!(!IndexSequencePartOne(-10, 0, 11, 21).is_vertical_in_bounds(&board));
        // test that a vertical sequence going off the board positively is rejected
        assert!(!generate_part_one_sequence(80, 10).is_vertical_in_bounds(&board));
        // test a vertical sequence starting at each first element is accepted.
        for start_index in 0..board.board_line_length {
            let seq = generate_part_one_sequence(start_index, board.board_line_length);
            assert!(seq.is_vertical_in_bounds(&board));
        }
    }

    #[test]
    fn test_is_horizontal_sequence_in_bound() {
        let board = build_test_board();
        // test that a horizontal sequence wrapping around the first column is rejected
        assert!(!generate_part_one_sequence(board.board_line_length + 1, -1)
            .is_horizontal_in_bounds(&board));
        // test that a horizontal sequence starting in the middle of a line is accepted
        assert!(generate_part_one_sequence(5, 1).is_horizontal_in_bounds(&board));
    }

    #[test]
    fn test_is_diagonal_sequence_in_bound() {
        let board = build_test_board();
        // test that a diagonal sequence starting from the first element column is accepted
        assert!(generate_part_one_sequence(0, board.board_line_length + 1)
            .is_diagonal_in_bounds(&board));
        // test that a diagonal sequence starting from the 3rd last element in the first row is
        // rejected because it would wrap around columns
        assert!(!generate_part_one_sequence(
            board.board_line_length - 3,
            board.board_line_length + 1
        )
        .is_diagonal_in_bounds(&board));
        // test that a backwards diagonal sequence starting from the 3rd element in the last row is
        // rejected because it would wrap around columns
        let seq = generate_part_one_sequence(
            board.board.len() as i32 - board.board_line_length + 2,
            -board.board_line_length - 1,
        );
        //println!("Seq: {:?}", seq);
        assert!(!seq.is_diagonal_in_bounds(&board));
        // test that a backwards diagonal sequence starting from the 2nd last element in the last row is
        // accepted because it is always in bounds
        let seq =
            generate_part_one_sequence(board.board.len() as i32 - 3, -board.board_line_length - 1);
        //println!("Seq: {:?}", seq);
        assert!(seq.is_diagonal_in_bounds(&board));
        // test that a backwards diagonal sequence starting from the middle of the 2nd row is rejected because it
        // would exit the array in negative bounds.
        assert!(!generate_part_one_sequence(
            board.board_line_length + 5,
            -board.board_line_length - 1
        )
        .is_diagonal_in_bounds(&board));
        // test that a diagonal sequence starting from the middle of the last row is rejected because it
        // would exit the array in positive bounds.
        assert!(!generate_part_one_sequence(
            board.board.len() as i32 - (board.board_line_length * 3) + 5,
            board.board_line_length + 1
        )
        .is_diagonal_in_bounds(&board));
    }

    #[test]
    fn test_generate_potential_matches() {
        let test_file = "mini_input".to_string();
        let board = build_board_from_file(&test_file);

        let potential_matches = generate_potential_matches_part_one(0, &board);
        assert_eq!(
            potential_matches,
            HashSet::from([
                IndexSequencePartOne(3, 2, 1, 0),
                IndexSequencePartOne(0, 1, 2, 3),
                IndexSequencePartOne(0, 4, 8, 12),
                IndexSequencePartOne(0, 5, 10, 15),
                IndexSequencePartOne(12, 8, 4, 0),
                IndexSequencePartOne(15, 10, 5, 0),
            ])
        );
        let potential_matches = generate_potential_matches_part_one(12, &board);
        assert_eq!(
            potential_matches,
            HashSet::from([
                IndexSequencePartOne(12, 8, 4, 0),
                IndexSequencePartOne(12, 13, 14, 15),
                IndexSequencePartOne(0, 4, 8, 12),
                IndexSequencePartOne(15, 14, 13, 12),
                IndexSequencePartOne(12, 9, 6, 3),
                IndexSequencePartOne(3, 6, 9, 12),
            ])
        );
    }

    #[test]
    fn test_evaluate_seq_for_match() {
        let board = build_board_from_file(&"mini_input".to_string());
        assert!(evaluate_seq_for_match(
            &IndexSequencePartOne(0, 1, 2, 3),
            &board
        ));
        assert!(evaluate_seq_for_match(
            &IndexSequencePartOne(12, 9, 6, 3),
            &board
        ));
        assert!(!evaluate_seq_for_match(
            &IndexSequencePartOne(0, 4, 8, 12),
            &board
        ));
    }

    #[test]
    fn test_evaluate_words() {
        let mut board = build_board_from_file(&"mini_input".to_string());
        let potential_matches = HashSet::from([
            IndexSequencePartOne(0, 1, 2, 3),
            IndexSequencePartOne(12, 9, 6, 3),
            IndexSequencePartOne(0, 4, 8, 12),
        ]);
        evaluate_words(potential_matches, &mut board);
        assert_eq!(
            board.matched_sequences_part_one,
            HashSet::from([
                IndexSequencePartOne(0, 1, 2, 3),
                IndexSequencePartOne(12, 9, 6, 3),
            ]),
        );
        assert_eq!(
            board.tested_sequences_part_one,
            HashSet::from([
                IndexSequencePartOne(0, 1, 2, 3),
                IndexSequencePartOne(12, 9, 6, 3),
                IndexSequencePartOne(0, 4, 8, 12)
            ]),
        );
    }

    #[test]
    fn test_is_vertical_sequence_in_bound_part_two() {
        let board = build_test_board();
        // test that in a sequence pair, either vertical sequence going off the
        // array bounds is rejected
        assert!(!XmasSequencePair(
            generate_part_two_sequence(80, 10),
            generate_part_two_sequence(-10, 5)
        )
        .is_vertical_in_bounds(&board));
        assert!(!XmasSequencePair(
            generate_part_two_sequence(8, -5),
            generate_part_two_sequence(10, board.board_line_length)
        )
        .is_vertical_in_bounds(&board));
        // test a vertical sequence starting at each first element is accepted.
        for start_index in 0..board.board_line_length {
            assert!(XmasSequencePair(
                generate_part_two_sequence(start_index, board.board_line_length),
                generate_part_two_sequence(start_index + 1, board.board_line_length)
            )
            .is_vertical_in_bounds(&board));
        }
    }

    #[test]
    fn test_is_diagonal_sequence_in_bound_part_two() {
        // Test that if either sequence in the XmasSequencePair goes out of vertical
        // bounds, then the sequence is rejected.
        let board = build_test_board();
        assert!(!XmasSequencePair(
            generate_part_two_sequence(5, -board.board_line_length - 1), // offending
            generate_part_two_sequence(0, board.board_line_length + 1),
        )
        .is_diagonal_in_bounds(&board));
        assert!(!XmasSequencePair(
            generate_part_two_sequence(0, board.board_line_length + 1),
            generate_part_two_sequence(90, board.board_line_length + 1), // offending
        )
        .is_diagonal_in_bounds(&board));

        // Test that if either sequence in the XmasSequencePair wraps around the
        // horizontal bounds of one column, the sequene is rejected.
        assert!(!XmasSequencePair(
            generate_part_two_sequence(0, 1),
            generate_part_two_sequence(board.board_line_length - 1, 1), // offending
        )
        .is_diagonal_in_bounds(&board));
        assert!(!XmasSequencePair(
            generate_part_two_sequence(board.board_line_length, -1), // offending
            generate_part_two_sequence(board.board_line_length, 1),
        )
        .is_diagonal_in_bounds(&board));
        assert!(!XmasSequencePair(
            generate_part_two_sequence(board.board_line_length + 1, -1), // offending
            generate_part_two_sequence(board.board_line_length, 1),
        )
        .is_diagonal_in_bounds(&board));
        assert!(!XmasSequencePair(
            generate_part_two_sequence(2 * board.board_line_length - 2, 1), // offending
            generate_part_two_sequence(board.board_line_length, 1),
        )
        .is_diagonal_in_bounds(&board));
    }
}
