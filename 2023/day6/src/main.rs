use std::env;
use roots::{Roots, find_roots_quadratic};

// Read argument for file input string.
fn get_file_name() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        return Err("No filename".to_string());
    }
    Ok(args[arg_len-1].clone())
}

// Process file input into a vector of pairs, each one describing race distance
// and time. e.g., for test input
// Time:      7  15   30
// Distance:  9  40  200
// [(7,9), (15,40), (30,200)]
fn read_input(fname: String) -> Vec<(u64, u64)> {
    let input_str = std::fs::read_to_string(fname).unwrap();

    // split into lines, and then get two vectors representing distance + time
    let lines: Vec<&str> = input_str.split('\n').collect();
    let parse_line_to_ivec = |s: &str| -> Vec<u64> {
        s.split([':', ' '])
        .filter_map(|num_candidate| {
            num_candidate.parse().ok()
        })
        .collect()
    };
    let time_vec: Vec<u64> = parse_line_to_ivec(lines.first().unwrap());
    let dist_vec: Vec<u64> = parse_line_to_ivec(lines.last().unwrap());
    assert_eq!(time_vec.len(), dist_vec.len());
    // UNCOMMENT FOR PART 1
    // time_vec.iter().enumerate().map(|e| {
    //     (e.1.clone(), dist_vec[e.0])
    // }).collect()

    // UNCOMMENT FOR PART 2
    // concatenate all these strings together
    let mut final_time = String::new();
    let mut final_dist = String::new();
    time_vec.iter().for_each(|s| final_time.push_str(&s.to_string()));
    dist_vec.iter().for_each(|s| final_dist.push_str(&s.to_string()));
    vec![(final_time.parse().unwrap(), final_dist.parse().unwrap())]
}

// calculate the product of all possible ways to beat the record in all races.
// Algorithm: build a quadratic equation for each race, and solve it for the roots.
// The number of wins for a race is the number of total solutions in the range of the
// two roots.
fn calc_wins(races: &Vec<(u64, u64)>) -> u64 {
    races.iter().filter_map(|race| -> Option<u64> {
        let t_lim = race.0;
        let d_lim = race.1;
        //println!("Solving the quadratic -x^2 + {}x - {} = 0", t_lim, d_lim);
        // return roots of the quadratic equation
        let eqn_offset: f64 = d_lim as f64;
        let roots = find_roots_quadratic::<f64>(-1.0, t_lim as f64, -1.0 as f64 * eqn_offset);

        let beats_record = |choice: u64| {
            if ((t_lim - choice) * choice) > d_lim {
                return true
            } else {
                return false
            }
        };

        match roots {
            Roots::No([]) => {
                //println!("No solutions, this race can't be won? Is that legit?");
                None
            },
            Roots::One([r1]) => {
                //println!("This race can only be won with one choice: {}", r1.ceil());
                if beats_record(r1.ceil() as u64) {
                    return Some(1);
                }
                None
            },
            Roots::Two([r1, r2]) => {
                let lower_bound = if beats_record(r1.ceil() as u64) {r1.ceil() as u64} else {r1.ceil() as u64 + 1};
                let upper_bound = if beats_record(r2.floor() as u64) {r2.floor() as u64} else {r2.floor() as u64 - 1};
                //println!("This race can be won with any choice between {} and {}!", lower_bound, upper_bound);
                Some(upper_bound - lower_bound + 1)
            },
            _ => unreachable!()
        }
    }).product()
}

fn main() {
    let win_margins = calc_wins(&read_input(get_file_name().unwrap()));
    println!("Total number of win possibilities = {}", win_margins)
}
