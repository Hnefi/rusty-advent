use std::fmt::Debug;

use commons::{arg_parsing::get_file_name_or_quit, io_utilities::read_file_to_string};

// GUARD_CHARS is organized in this exact order on purpose, so that we can calculate the next guard
// direction on turns by finding the index in the array and then calling next() to rotate it
// clockwise by 90 degrees.
const GUARD_CHARS_LEN: usize = 4;
const GUARD_CHARS_LAST_IDX: usize = GUARD_CHARS_LEN - 1;
const GUARD_CHARS: [char; GUARD_CHARS_LEN] = ['^', '>', 'v', '<'];
const VISITED_POS: char = 'X';

#[derive(Debug)]
enum NextGuardAction {
    Advance,
    Turn,
    ExitLab,
}

pub trait GuardCharAccessors {
    fn get_guard_character(&self) -> char;
    fn get_guard_char_reference(&mut self) -> &mut char;
    fn get_next_index(&self, offset: i64) -> i64;
}

pub struct Lab {
    map: Vec<char>,
    line_length: usize,
    guard_position: usize,
}

impl Debug for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string = String::new();
        // Iterate over the self map by chunks of 'line_length' and write them to the formatter.
        self.map.chunks(self.line_length).for_each(|chunk| {
            let chunk_as_string: Vec<String> = chunk.iter().map(|c| c.to_string()).collect();
            map_string.push_str(&format!("{}\n", chunk_as_string.concat()));
        });
        let _ = f.debug_struct("Lab:\n").finish();
        write!(f, "{}", map_string)
    }
}

impl GuardCharAccessors for Lab {
    fn get_guard_character(&self) -> char {
        self.map[self.guard_position]
    }

    fn get_guard_char_reference(&mut self) -> &mut char {
        self.map.get_mut(self.guard_position).unwrap()
    }

    fn get_next_index(&self, offset: i64) -> i64 {
        self.guard_position as i64 + offset
    }
}

fn map_and_line_length_from_raw_string(raw_string: &str) -> (Vec<char>, usize) {
    (
        raw_string.chars().filter(|&c| c != '\n').collect(),
        raw_string.lines().next().unwrap().len(),
    )
}

fn map_and_line_length_from_file(fname: &String) -> (Vec<char>, usize) {
    let raw_string = read_file_to_string(fname);
    map_and_line_length_from_raw_string(&raw_string)
}

fn build_lab_from_file(fname: &String) -> Lab {
    let (map, line_length) = map_and_line_length_from_file(fname);
    let current_pos = find_starting_guard_pos(&map);
    Lab {
        map,
        line_length,
        guard_position: current_pos,
    }
}

fn find_starting_guard_pos(map: &[char]) -> usize {
    let current_pos = map
        .iter()
        .enumerate()
        .filter_map(|(idx, c)| {
            if GUARD_CHARS.contains(c) {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    // Assumes only 1 guard char in the input, which is fine.
    current_pos
}

fn mark_position_visited(lab: &mut Lab, position: usize) {
    lab.map[position] = 'X';
}

fn next_step_is_blocked(lab: &Lab, next_step_offset: i64) -> bool {
    lab.map[lab.get_next_index(next_step_offset) as usize] == '#'
}

fn next_step_exits_lab(lab: &Lab, next_step_offset: i64) -> bool {
    let idx = lab.get_next_index(next_step_offset);
    //println!("Testing idx = {idx}");
    if idx >= lab.map.len() as i64 || idx < 0 {
        return true;
    }
    if ['<', '>'].contains(&lab.get_guard_character()) {
        let rows: Vec<i64> = [lab.guard_position as i64, idx]
            .into_iter()
            .map(|pos| pos / lab.line_length as i64)
            .collect();
        //println!("Calculated rows = {:?}", rows);
        let first_row = rows.first().unwrap();
        if rows.iter().any(|elem| elem != first_row) {
            return true;
        }
    }
    false
}

fn next_guard_action(lab: &Lab) -> NextGuardAction {
    let possible_next_step = match lab.get_guard_character() {
        '^' => -(lab.line_length as i64),
        'v' => lab.line_length as i64,
        '>' => 1,
        '<' => -1,
        _ => unreachable!("Impossible guard character!!"),
    };
    if next_step_exits_lab(lab, possible_next_step) {
        return NextGuardAction::ExitLab;
    } else if next_step_is_blocked(lab, possible_next_step) {
        return NextGuardAction::Turn;
    }
    NextGuardAction::Advance
}

// Move the guard one step in the direction corresponding to its character, and return the old
// guard position so that it can then be marked as visited by the caller.
fn advance_guard(lab: &mut Lab) -> usize {
    let old_position = lab.guard_position;
    let current_guard_char = lab.map[old_position];
    let offset = match current_guard_char {
        '^' => -(lab.line_length as i64),
        '>' => 1,
        '<' => -1_i64,
        'v' => lab.line_length as i64,
        _ => unreachable!("Impossible guard character!!"),
    };
    lab.guard_position = (lab.guard_position as i64 + offset) as usize;
    *lab.get_guard_char_reference() = *GUARD_CHARS
        .iter()
        .find(|c| **c == current_guard_char)
        .unwrap();
    old_position
}

fn rotate_guard(lab: &mut Lab) {
    let idx = GUARD_CHARS
        .iter()
        .position(|c| *c == lab.map[lab.guard_position])
        .unwrap(); // will throw if the guard char isn't found, which is a fatal error
    let next_idx = match idx {
        GUARD_CHARS_LAST_IDX => 0,
        _ => idx + 1,
    };
    lab.map[lab.guard_position] = GUARD_CHARS[next_idx];
}

fn calculate_guard_path(lab: &mut Lab) {
    loop {
        // Calculate the guard's path based on its current location and direction, and take the
        // correct action (e.g., advance, turn, exit).
        match next_guard_action(lab) {
            NextGuardAction::Advance => {
                let old_position = advance_guard(lab);
                mark_position_visited(lab, old_position);
            }
            NextGuardAction::Turn => {
                rotate_guard(lab);
            }
            NextGuardAction::ExitLab => {
                // mark the guard's current position as visited (because they're on the edge, they
                // will go out of the lab) and then exit the loop
                mark_position_visited(lab, lab.guard_position);
                break;
            }
        }
        //println!("--- Iteration done ---\n{:?}", lab);
    }
}

fn main() {
    let fname = get_file_name_or_quit();
    println!("AOC Day6 - Parsing input {fname}...");
    let mut lab = build_lab_from_file(&fname);
    //println!("Lab: {:?}", lab);
    println!("Parsed {:?}. Calculating guard positions....", lab);
    calculate_guard_path(&mut lab);
    println!(
        "Done! Guard visited {} different positions!",
        lab.map.iter().filter(|&pos| *pos == VISITED_POS).count()
    );
}

fn build_test_lab() -> Lab {
    let test_file = "test_input".to_string();
    build_lab_from_file(&test_file)
}

fn build_test_lab_from_string(test_string: &str) -> Lab {
    let (map, line_length) = map_and_line_length_from_raw_string(test_string);
    let current_pos = find_starting_guard_pos(&map);

    Lab {
        map,
        line_length,
        guard_position: current_pos,
    }
}

#[cfg(test)]
mod tests {
    use super::*; // bring in all the functionality in the word_search module

    #[test]
    fn test_next_step_is_blocked() {
        // Test a basic case for being blocked when going left, right, up, and down.
        let lab = build_test_lab_from_string("...\n.>#\n...\n");
        assert!(next_step_is_blocked(&lab, 1));
        let lab = build_test_lab_from_string("...\n.>.\n...\n");
        assert!(!next_step_is_blocked(&lab, 1));
        let lab = build_test_lab_from_string("...\n.v.\n.#.\n");
        assert!(next_step_is_blocked(
            &lab,
            lab.line_length.try_into().unwrap()
        ));
        let lab = build_test_lab_from_string("...\n.v.\n...\n");
        assert!(!next_step_is_blocked(
            &lab,
            lab.line_length.try_into().unwrap()
        ));
        let lab = build_test_lab_from_string(".#.\n.^.\n...\n");
        assert!(next_step_is_blocked(&lab, -(lab.line_length as i64)));
        let lab = build_test_lab_from_string("...\n.^.\n...\n");
        assert!(!next_step_is_blocked(&lab, -(lab.line_length as i64)));
        let lab = build_test_lab_from_string("#<.\n...\n...\n");
        assert!(next_step_is_blocked(&lab, -1));
        let lab = build_test_lab_from_string(".<.\n...\n...\n");
        assert!(!next_step_is_blocked(&lab, -1));
    }

    #[test]
    fn test_next_step_exits_lab() {
        let lab = build_test_lab_from_string(".^.\n.##\n...\n");
        assert!(next_step_exits_lab(&lab, -(lab.line_length as i64)));
        let lab = build_test_lab_from_string("...\n.^#\n...\n");
        assert!(!next_step_exits_lab(&lab, -(lab.line_length as i64)));
        let lab = build_test_lab_from_string("..>\n.##\n...\n");
        assert!(next_step_exits_lab(&lab, 1));
        let lab = build_test_lab_from_string("...\n.##\n..>\n");
        assert!(next_step_exits_lab(&lab, 1));
        let lab = build_test_lab_from_string("...\n.##\n.>.\n");
        assert!(!next_step_exits_lab(&lab, 1));
        let lab = build_test_lab_from_string("<..\n.##\n...\n");
        assert!(next_step_exits_lab(&lab, -1));
        let lab = build_test_lab_from_string(".<.\n.##\n...\n");
        assert!(!next_step_exits_lab(&lab, -1));
        let lab = build_test_lab_from_string("...\n.v#\n...\n");
        assert!(!next_step_exits_lab(&lab, lab.line_length as i64));
        let lab = build_test_lab_from_string("...\n..#\n.v.\n");
        assert!(next_step_exits_lab(&lab, lab.line_length as i64));
    }

    #[test]
    fn test_rotate_guard() {
        let mut lab = build_test_lab();
        // Loop over all guard positions and ensure that rotating always gives the correct
        // next direction.
        for c in GUARD_CHARS.iter() {
            match c {
                '>' => {
                    *lab.get_guard_char_reference() = *c;
                    rotate_guard(&mut lab);
                    assert_eq!(lab.get_guard_character(), 'v');
                }
                '<' => {
                    *lab.get_guard_char_reference() = *c;
                    rotate_guard(&mut lab);
                    assert_eq!(lab.get_guard_character(), '^');
                }
                '^' => {
                    *lab.get_guard_char_reference() = *c;
                    rotate_guard(&mut lab);
                    assert_eq!(lab.get_guard_character(), '>');
                }
                'v' => {
                    *lab.get_guard_char_reference() = *c;
                    rotate_guard(&mut lab);
                    assert_eq!(lab.get_guard_character(), '<');
                }
                _ => unreachable!("Impossible guard character."),
            }
        }
    }

    #[test]
    fn test_advance_guard() {
        // Loop over all guard positions and ensure that regardless of the initial guard direction,
        // it advances by one step and the old position is marked as "X".
        for c in GUARD_CHARS.iter() {
            let mut lab = build_test_lab();
            match c {
                '^' => {
                    // Set current guard direction to ^ and then test moving the guard up one line.
                    *lab.get_guard_char_reference() = *c;
                    let expected_new_pos = lab.guard_position - lab.line_length;
                    let old_pos = advance_guard(&mut lab);
                    assert_eq!(lab.map[expected_new_pos], '^');
                    assert_eq!(lab.map[old_pos], '^'); // The old position must not be changed to
                                                       // 'X' yet.
                }
                '>' => {
                    // Set current guard direction to ^ and then test moving the guard up one line.
                    *lab.get_guard_char_reference() = *c;
                    let expected_new_pos = lab.guard_position + 1;
                    let old_pos = advance_guard(&mut lab);
                    assert_eq!(lab.map[expected_new_pos], '>');
                    assert_eq!(lab.map[old_pos], '>'); // The old position must not be changed to
                                                       // 'X' yet.
                }
                '<' => {
                    // Set current guard direction to ^ and then test moving the guard up one line.
                    *lab.get_guard_char_reference() = *c;
                    let expected_new_pos = lab.guard_position - 1;
                    let old_pos = advance_guard(&mut lab);
                    assert_eq!(lab.map[expected_new_pos], '<');
                    assert_eq!(lab.map[old_pos], '<'); // The old position must not be changed to
                                                       // 'X' yet.
                }
                'v' => {
                    // Set current guard direction to ^ and then test moving the guard up one line.
                    *lab.get_guard_char_reference() = *c;
                    let expected_new_pos = lab.guard_position + lab.line_length;
                    let old_pos = advance_guard(&mut lab);
                    assert_eq!(lab.map[expected_new_pos], 'v');
                    assert_eq!(lab.map[old_pos], 'v'); // The old position must not be changed to
                                                       // 'X' yet.
                }
                _ => unreachable!("Impossible guard character."),
            }
        }
    }

    #[test]
    fn test_mark_position_visited() {
        let mut lab = build_test_lab();
        mark_position_visited(&mut lab, 0);
        assert_eq!(*lab.map.first().unwrap(), 'X');
    }
}
