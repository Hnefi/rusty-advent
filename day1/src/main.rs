use std::env;
use std::fs::read_to_string;

// Read argument for file input string.
fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    return args[arg_len-1].clone();
}

// Open file and return a vector of all its lines.
fn get_file_lines(file_name: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file_string = read_to_string(file_name);
    for line in file_string.unwrap().lines() {
        result.push(line.to_string());
    }
    return result;
}

// Check if a given character is a digit.
fn is_digit(a: char) -> bool {
    match a {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => return true,
        _ => return false
    }
}

// For a given string, get the sum of the first and last digits appearing in it.
fn get_digit_sum(s: &String) -> u32 {
    let mut sum: u32 = 0;
    for _char in s.chars() {
        if is_digit(_char) {
            sum += 10*(_char.to_digit(10).unwrap());
            //println!("Found first digit, adding {} to sum. Sum = {}", _char, sum);
            break;
        }
    }
    
    for _char in s.chars().rev() {
        if is_digit(_char) {
            sum += _char.to_digit(10).unwrap();
            //println!("Found last digit, adding {} to sum. Sum = {}", _char, sum);
            break;
        }
    }
    return sum;
}

fn main() {
    let file_lines = get_file_lines(get_file_name());
    let mut sum = 0;
    for line in file_lines.iter() {
        //println!("Line to test = {}", line);
        sum += get_digit_sum(line);
    }
    //println!("Final result was: {}",sum);
}
