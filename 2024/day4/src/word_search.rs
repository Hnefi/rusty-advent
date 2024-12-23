//pub mod word_search {
use commons::io_utilities::read_file_lines;
use std::{cmp::Ordering, collections::HashSet};

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct IndexSequence(pub i32, pub i32, pub i32, pub i32);

impl IndexSequence {
    pub fn as_array(&self) -> [i32; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn is_forward_match(self) -> bool {
        return (self.0 < self.1);
    }

    pub fn is_reverse_match(self) -> bool {
        return (self.0 > self.1);
    }
}

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
    pub matched_sequences: HashSet<IndexSequence>,
    pub tested_sequences: HashSet<IndexSequence>,
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
        matched_sequences: HashSet::new(),
        tested_sequences: HashSet::new(),
    }
}

pub fn generate_sequence(base_index: i32, delta: i32) -> IndexSequence {
    IndexSequence(
        base_index,
        base_index + delta,
        base_index + 2 * delta,
        base_index + 3 * delta,
    )
}

pub fn add_new_sequence_if_untested(
    seq: IndexSequence,
    board: &WordSearchBoard,
    sequences: &mut HashSet<IndexSequence>,
) -> bool {
    if !board.tested_sequences.contains(&seq) {
        sequences.insert(seq);
        return true;
    }
    false
}

fn sequence_in_array_bounds(seq: &IndexSequence, board: &WordSearchBoard) -> bool {
    if seq
        .as_array()
        .into_iter()
        .any(|val| val < 0 || val > board.board.len().try_into().unwrap())
    {
        return false;
    }
    true
}

fn horizontal_sequence_in_bounds(seq: &IndexSequence, board: &WordSearchBoard) -> bool {
    if sequence_in_array_bounds(seq, board) {
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
    }
    false
}

fn diagonal_sequence_in_bounds(seq: &IndexSequence, board: &WordSearchBoard) -> bool {
    // Since diagonal sequences are vertical, use the existing "vertical_in_bounds" fn.
    if sequence_in_array_bounds(seq, board) {
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
    }
    false
}

pub fn generate_potential_matches(
    base_index: i32,
    board: &WordSearchBoard,
) -> HashSet<IndexSequence> {
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
        let seq: IndexSequence = generate_sequence(start, board.board_line_length);
        //println!("Generated seq {:?}", seq);
        if sequence_in_array_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 2: horizontal sequences.
        let start = base_index + off;
        let seq: IndexSequence = generate_sequence(start, 1);
        //println!("Generated seq {:?}", seq);
        if horizontal_sequence_in_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 3: diagonal sequences.
        let start = base_index + (board.board_line_length * off) + off;
        let seq: IndexSequence = generate_sequence(start, board.board_line_length + 1);
        //println!("Generated seq {:?}", seq);
        if diagonal_sequence_in_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }
    });

    positive_offsets.into_iter().for_each(|off| {
        // Type 1: vertical sequences.
        let start = base_index + (board.board_line_length * off);
        let seq: IndexSequence = generate_sequence(start, -board.board_line_length);
        //println!("Generated seq {:?}", seq);
        if sequence_in_array_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 2: horizontal sequences.
        let start = base_index + off;
        let seq: IndexSequence = generate_sequence(start, -1);
        //println!("Generated seq {:?}", seq);
        if horizontal_sequence_in_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }

        // Type 3: diagonal sequences.
        let start = base_index + (board.board_line_length * off) + off;
        let seq: IndexSequence = generate_sequence(start, -board.board_line_length - 1);
        //println!("Generated seq {:?}", seq);
        if diagonal_sequence_in_bounds(&seq, board) {
            //println!("Added {:?}", seq);
            add_new_sequence_if_untested(seq, board, &mut matches);
        }
    });
    matches
}

pub fn evaluate_words(potential_matches: HashSet<IndexSequence>, mut board: &WordSearchBoard) {}

#[cfg(test)]
mod tests {
    use super::*; // bring in all the functionality in the word_search module

    #[test]
    fn test_generate_sequence() {
        let base = 4;
        let test_positive_delta = 2;
        assert_eq!(
            generate_sequence(base, test_positive_delta),
            IndexSequence(4, 6, 8, 10)
        );
        let test_negative_delta = -3;
        assert_eq!(
            generate_sequence(base, test_negative_delta),
            IndexSequence(4, 1, -2, -5)
        );
    }

    #[test]
    fn test_add_new_sequence_if_untested() {
        let mut board = build_test_board();
        let new_seq = IndexSequence(1, 2, 3, 4);
        let mut sequences_to_test: HashSet<IndexSequence> = HashSet::new();
        // assert new sequence is added to sequences_to_test
        assert!(add_new_sequence_if_untested(
            new_seq,
            &board,
            &mut sequences_to_test
        ));
        // assert vec has the new_seq value
        assert!(sequences_to_test.contains(&new_seq));

        board.tested_sequences.insert(new_seq);
        // assert trying to add the same sequence now is false
        assert!(!add_new_sequence_if_untested(
            new_seq,
            &board,
            &mut sequences_to_test
        ));
    }

    #[test]
    fn test_is_vertical_sequence_in_bound() {
        let board = build_test_board();
        // test that a vertical sequence going off the board negatively is rejected
        assert!(!sequence_in_array_bounds(
            &IndexSequence(-10, 0, 11, 21),
            &board
        ));
        // test that a vertical sequence going off the board positively is rejected
        assert!(!sequence_in_array_bounds(
            &generate_sequence(80, 10),
            &board
        ));
        // test a vertical sequence starting at each first element is accepted.
        for start_index in 0..board.board_line_length {
            let seq = generate_sequence(start_index, board.board_line_length);
            assert!(sequence_in_array_bounds(&seq, &board));
        }
    }

    #[test]
    fn test_is_horizontal_sequence_in_bound() {
        let board = build_test_board();
        // test that a horizontal sequence wrapping around the first column is rejected
        assert!(!horizontal_sequence_in_bounds(
            &generate_sequence(board.board_line_length + 1, -1),
            &board
        ));
        // test that a horizontal sequence starting in the middle of a line is accepted
        assert!(horizontal_sequence_in_bounds(
            &generate_sequence(5, 1),
            &board
        ))
    }

    #[test]
    fn test_is_diagonal_sequence_in_bound() {
        let board = build_test_board();
        // test that a diagonal sequence starting from the first element column is accepted
        assert!(diagonal_sequence_in_bounds(
            &generate_sequence(0, board.board_line_length + 1),
            &board
        ));
        // test that a diagonal sequence starting from the 3rd last element in the first row is
        // rejected because it would wrap around columns
        assert!(!diagonal_sequence_in_bounds(
            &generate_sequence(board.board_line_length - 3, board.board_line_length + 1),
            &board
        ));
        // test that a backwards diagonal sequence starting from the 3rd element in the last row is
        // rejected because it would wrap around columns
        let seq = generate_sequence(
            board.board.len() as i32 - board.board_line_length + 2,
            -board.board_line_length - 1,
        );
        //println!("Seq: {:?}", seq);
        assert!(!diagonal_sequence_in_bounds(&seq, &board));
        // test that a backwards diagonal sequence starting from the 2nd last element in the last row is
        // accepted because it is always in bounds
        let seq = generate_sequence(board.board.len() as i32 - 3, -board.board_line_length - 1);
        //println!("Seq: {:?}", seq);
        assert!(diagonal_sequence_in_bounds(&seq, &board));
        // test that a backwards diagonal sequence starting from the middle of the 2nd row is rejected because it
        // would exit the array in negative bounds.
        assert!(!diagonal_sequence_in_bounds(
            &generate_sequence(board.board_line_length + 5, -board.board_line_length - 1),
            &board
        ));
        // test that a diagonal sequence starting from the middle of the last row is rejected because it
        // would exit the array in positive bounds.
        assert!(!diagonal_sequence_in_bounds(
            &generate_sequence(
                board.board.len() as i32 - (board.board_line_length * 3) + 5,
                board.board_line_length + 1
            ),
            &board
        ));
    }

    #[test]
    fn test_generate_potential_matches() {
        let test_file = "mini_input".to_string();
        let board = build_board_from_file(&test_file);

        let potential_matches = generate_potential_matches(0, &board);
        assert_eq!(
            potential_matches,
            HashSet::from([
                IndexSequence(3, 2, 1, 0),
                IndexSequence(0, 1, 2, 3),
                IndexSequence(0, 4, 8, 12),
                IndexSequence(0, 5, 10, 15),
                IndexSequence(12, 8, 4, 0),
                IndexSequence(15, 10, 5, 0),
            ])
        );
    }
}
