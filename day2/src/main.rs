use std::env;
use std::fs::read_to_string;
use regex::Regex;

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

// A struct representing a draw, having a given number for all balls.
#[derive(Clone)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32
}

// A struct representing a game, having an id and list of draws.
struct Game {
    id: i32,
    draws: Vec<Draw>
}

impl Game {
    fn return_id_if_valid(&self) -> i32 {
        // Constants provided by the problem statement.
        const RED_PROVIDED: i32 = 12;
        const BLUE_PROVIDED: i32 = 14;
        const GREEN_PROVIDED: i32 = 13;
        match self.draws.iter().all(|draw| {
            //println!("Draw contined {} red, {} blue, and {} green", draw.red, draw.blue, draw.green);
            if draw.red <= RED_PROVIDED && draw.green <= GREEN_PROVIDED && draw.blue <= BLUE_PROVIDED {
                return true;
            } else {
                return false;
            }
        }) {
            true => return self.id,
            false => return 0
        }
    }
}

// Parse a line of text and build a game out of it.
fn parse_game(input_string: String) -> Game {
    //println!("Matching on input string {}", input_string);
    // get the game id with a regex
    let re = Regex::new(r"Game (\d+).*").unwrap();
    let Some(capture) = re.captures(&input_string) else {
        println!("No game id could be captured!!");
        quit::with_code(1);
    };
    let game_id: i32 = (&capture[1]).parse().unwrap();

    // split line by semicolons and match each one on a regex for the numbers and colors
    let v: Vec<&str> = input_string.split(';').collect();
    let sub_draw_re = Regex::new(r"(\d+)\s([[:alpha:]]+)").unwrap();

    let mut draw_vec: Vec<Draw> = Vec::new();    
    for split in v.iter() {
        //println!("Raw split={}", split);
        let mut num_blue = 0;
        let mut num_green = 0;
        let mut num_red = 0;
        let re_draws = Regex::new(r"[:,\s]*(\d+\s[[:alpha:]]+)").unwrap();
        for (_, [sub_draw]) in re_draws.captures_iter(&split).map(|c| c.extract()) {
            //println!("full {}: digit={}",full, sub_draw);
            for (_, [num_balls, color]) in sub_draw_re.captures_iter(sub_draw).map(|c| c.extract()) {
                //println!("num_balls {}, color {}",num_balls, color);
                match color {
                    "red" => num_red = num_balls.parse().unwrap(),
                    "blue" => num_blue = num_balls.parse().unwrap(),
                    "green" => num_green = num_balls.parse().unwrap(),
                    _ => quit::with_code(1)
                }
            }
        }
        //println!("adding new draw: r{}, g{}, b{}", num_red, num_green, num_blue);
        draw_vec.push(Draw{red: num_red, green: num_green, blue: num_blue});
    }

    // Iterate over all the draws in the line and create draws for them.
    return Game {id: game_id, draws: draw_vec}
}

fn main() {
    let file_lines = get_file_lines(get_file_name());
    let game_sum: i32 = file_lines.iter().map(|line| {
        parse_game(line.to_string()).return_id_if_valid()
    }).sum();
    println!("Sum of games = {}", game_sum);
}