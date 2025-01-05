use commons::{arg_parsing::get_file_name_or_quit, io_utilities::read_file_to_string};

const GUARD_CHARS: [char; 4] = ['^', 'v', '>', '<'];

#[derive(Debug)]
struct Lab {
    map: Vec<char>,
    line_length: usize,
    guard_position: usize,
}

fn build_map(fname: &String) -> (Vec<char>, usize) {
    let raw_string = read_file_to_string(fname);
    (
        raw_string.chars().filter(|&c| c != '\n').collect(),
        raw_string.lines().next().unwrap().len(),
    )
}

fn build_lab(fname: &String) -> Lab {
    let (map, line_length) = build_map(fname);
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
    Lab {
        map,
        line_length,
        guard_position: current_pos,
    }
}

fn main() {
    let fname = get_file_name_or_quit();
    println!("AOC Day6 - Parsing input {fname}...");
    let lab = build_lab(&fname);
    //println!("Lab: {:?}", lab);
}
