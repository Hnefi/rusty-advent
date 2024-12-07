use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

// Doesn't work, but I learned a lot of string syntax.

// Substitute the first and last instances of a written-out numbers, returning
// another string with the substitutions.
fn replace_written_numbers(input_string: String) -> String {
    let mut output_str = input_string.clone();
    let three_letter_replacements: HashMap<&str, &str> = HashMap::from(
        [
            ("one", "1"),
            ("two", "2"),
            ("six", "6")
        ]
    );
    let four_letter_replacements: HashMap<&str, &str> = HashMap::from(
        [
            ("zero", "0"),
            ("four", "4"),
            ("five", "5"),
            ("nine", "9")
        ]
    );
    let five_letter_replacements: HashMap<&str, &str> = HashMap::from(
        [
            ("three", "3"),
            ("seven", "7"),
            ("eight", "8"),
        ]
    );

    fn check_and_replace(mutable_string: &mut String, range: RangeInclusive<usize> , replacement_map: &HashMap<&str, &str>) -> bool { 
        //println!("Checking at slice {}", &mutable_string[range.clone()]);
        match replacement_map.contains_key(&mutable_string[range.clone()]) {
            true => {
                mutable_string.replace_range(range.clone(), replacement_map.get(&mutable_string[range]).unwrap());
                return true;
            }
            false => return false,
        }
    }

    println!("Old string {}", input_string);
    let mut found: bool = false;
    if output_str.len() >= 5 {
        // Slide a window over the string forwards, finding the first matched written digit
        let end_idx = output_str.len() - 5;
        for idx in 0..=end_idx {
            if check_and_replace(&mut output_str, idx..=idx+2, &three_letter_replacements) {
                found = true;
                break;
            } else if check_and_replace(&mut output_str, idx..=idx+3, &four_letter_replacements) {
                found = true;
                break;
            } else if check_and_replace(&mut output_str, idx..=idx+4, &five_letter_replacements) {
                found = true;
                break;
            }
        }
    }
    // Need to check at the 4th last character, for 3+4 letter subs
    if !found && output_str.len() >= 4 {
        let idx = output_str.len()-4;
        if check_and_replace(&mut output_str, idx..=idx+2, &three_letter_replacements) {
            found = true;
        } else if check_and_replace(&mut output_str, idx..=idx+3, &four_letter_replacements) {
            found = true;
        }
    }
    if !found && output_str.len() >= 3 {
        // check finally at the 3rd last character
        let idx = output_str.len() - 3;
        check_and_replace(&mut output_str, idx..=idx+2, &three_letter_replacements);
    }
    println!("New string {}", output_str);

    let mut found_reverse: bool = false;
    // Prologue to check the 3/4 letter substitutions from the end
    if output_str.len() >= 3 {
        let end_idx = output_str.len() - 3;
        if check_and_replace(&mut output_str, end_idx..=end_idx+2, &three_letter_replacements) {
            found_reverse = true;
        }
    }
    if !found_reverse && output_str.len() >= 4 {
        let end_idx = output_str.len() - 4;
        if check_and_replace(&mut output_str, end_idx..=end_idx+3, &four_letter_replacements) {
            found_reverse = true;
        } else if check_and_replace(&mut output_str, end_idx..=end_idx+2, &three_letter_replacements) {
            found_reverse = true;
        }
        if !found_reverse && output_str.len() >= 5 {
            // Slide a window over the string backwards , finding the first matched written digit
            for idx in (0..=output_str.len()-5).rev() {
                //println!("Starting to check from idx {}", idx);
                if check_and_replace(&mut output_str, idx..=idx+4, &five_letter_replacements) {
                    break;
                } else if check_and_replace(&mut output_str, idx..=idx+3, &four_letter_replacements) {
                    break;
                } else if check_and_replace(&mut output_str, idx..=idx+2, &three_letter_replacements) {
                    break;
                }
            }
        }
    }
    println!("New string after reverse sub: {}", output_str);
    output_str
}

// Read argument for file input string.
fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len-1].clone()
}

// Open file and return a vector of all its lines.
fn get_file_lines(file_name: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file_string = read_to_string(file_name);
    for line in file_string.unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

// Check if a given character is a digit.
fn is_digit(a: char) -> bool {
    match a {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false
    }
}

// For a given string, get the sum of the first and last digits appearing in it.
fn get_digit_sum(s: &String) -> u32 {
    let mut sum: u32 = 0;
    for _char in s.chars() {
        if is_digit(_char) {
            sum += 10*(_char.to_digit(10).unwrap());
            break;
        }
    }
    
    for _char in s.chars().rev() {
        if is_digit(_char) {
            sum += _char.to_digit(10).unwrap();
            break;
        }
    }
    sum
}

fn main() {
    let file_lines = get_file_lines(get_file_name());
    let mut sum = 0;
    for line in file_lines.iter() {
        println!("Line to test = {}", line);
        sum += get_digit_sum(&replace_written_numbers(line.to_string()));
    }
    println!("Final result was: {}",sum);
}
