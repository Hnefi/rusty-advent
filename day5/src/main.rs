use std::{collections::HashMap, env, ops::RangeInclusive};
use std::cmp::{max,min};

// Read argument for file input string.
fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}

// Return a tuple containing:
// - whether this range was transformed
// - a transformed range (optional)
// - a remainder range (optional)
fn overlap_and_exclusive(input: &RangeInclusive<i64>, source: &RangeInclusive<i64>, offset: i64)
    -> (Option<RangeInclusive<i64>>, Option<RangeInclusive<i64>>) {
    //println!("Checking input range {:?} against source transform {:?}",input, source);
    if source.contains(input.start()) && source.contains(input.end()) {
        //println!("Input range {:?} completely contained in source {:?}", input, source);
        // .... S1 -------------- Sn ....
        // ........ I1............In ....
        // .........O1............On .....
        let overlap_start = max(source.start(), input.start());
        let overlap_end = min(source.end(), input.end());
        // result.push(RangeInclusive::new(
        //     overlap_start+offset,
        //     overlap_end+offset
        // ));
        return (Some(RangeInclusive::new(
            overlap_start+offset,
            overlap_end+offset)), None);
    } else if source.contains(input.start()) && !source.contains(input.end()) {
        //println!("Input range {:?} partially overlaps source {:?} on the upper side", input, source);
        // .... S1 -------------- Sn ....
        // ................ I1................In ....
        // Output: .........O1....On On+1.....In ....
        let transformed_range = RangeInclusive::new(input.start()+offset, source.end()+offset);
        let rem_range = RangeInclusive::new(source.end()+1, *input.end());
        return (Some(transformed_range), Some(rem_range));
    } else if !source.contains(input.start()) && source.contains(input.end()) {
        //println!("Input range {:?} partially overlaps source {:?} on the lower side", input, source);
        // ................. S1 -------------- Sn ....
        // ........ I1................In ....
        // Output: .I1..S1-1.S1 ......In ............
        let rem_range = RangeInclusive::new(*input.start(), source.start()-1);
        let transformed_range = RangeInclusive::new(source.start()+offset, input.end()+offset);
        return (Some(transformed_range), Some(rem_range));
    } else {
        //println!("No overlap between the two ranges");
        return (None, None)
    }
}

fn convert_seeds(input: &Vec<&str>) -> i64 {
    // algorithm:
    // - first build the list of seeds (input)
    // - then, for every following "map", build a HashSet containing the mappings of src->dest id
    // - convert the seed list to the next representation, and finally output the minimum

    // Parse the seeds in the first line
    let seed_string: Vec<&str> = input[1].split('\n').collect();
    //println!("Seed string = {:?}",seed_string);
    // PART 1 - individual seeds
    // let mut seeds: Vec<i64> = seed_string.first().unwrap()
    //     .trim()
    //     .split(" ")
    //     .map(|c| c.parse().unwrap())
    //     .collect();

    // PART 2 - Seed ranges
    let seed_seeds: Vec<i64> = seed_string
        .first()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut seed_ranges: Vec<RangeInclusive<i64>> = Vec::new();
    (0..seed_seeds.len()/2).for_each(|idx| {
        seed_ranges.push(RangeInclusive::new(
            // -1 for using an inclusive range. 0 10 -> [0..9]
            seed_seeds[2 * idx], seed_seeds[2*idx] + seed_seeds[2 * idx + 1]-1));
    });
    //println!("Seed ranges: {:?}", seed_ranges);

    //
    // Algorithm:
    // - for each "mapping" of seeds in the problem input set, create two other sets of ranges
    //   like this: {[begin, end], offset}
    //   e.g., {[98, 99], -48}, {[50, 97], 2}
    let mut seed_map: HashMap<RangeInclusive<i64>, i64> = HashMap::new();
    for idx in 2..input.len() {
        //println!("Map: {:?}", input[idx])
        let mappings: Vec<&str> = input[idx].split('\n').map(|s| s.trim()).collect();
        //println!("Mappings: {:?}", mappings);
        mappings.iter().for_each(|m| {
            let range = m
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i64>>();
            if range.len() == 3 {
                // hack
                let (dst, src, len) = match &range[..] {
                    &[first, second, third, ..] => (first, second, third),
                    _ => unreachable!(),
                };
                // Build the source range.
                // -1 for using an inclusive range.
                let source_range = RangeInclusive::new(src, src+len-1);
                // seed map contains an "offset"
                let offset: i64 = dst - src;
                // println!(
                //     "Inserting source range w/ start {}, end {}, with an offset of {}",
                //     source_range.start(), source_range.end(), offset,
                // );
                seed_map.insert(source_range, offset);
            }
        });

        // - then, map the input seed ranges to a set of new ranges.
        // - given input [90, 99], and an input of {[98, 99], -48}, {[50, 97], 2}
        //   we would want the output as [92, 99], [50, 51]
        //println!("Seed ranges before: {:?}", seed_ranges);

        let mut next_step_ranges: Vec<RangeInclusive<i64>> = Vec::new();
        let mut remainder_ranges: Vec<RangeInclusive<i64>> = Vec::new();
        let mut ranges_to_consume = true;
        while ranges_to_consume {
            let mut transformed_ranges: Vec<RangeInclusive<i64>> = seed_ranges.iter().flat_map(|input_range| {
                // return the range overlap between the src and the current range, and then
                // translate it to the destination
                let mut output_ranges: Vec<RangeInclusive<i64>> = Vec::new();
                let mut found = false;
                for (src, offset) in seed_map.iter() {
                    let (transformed_range, remainder_range) = overlap_and_exclusive(&input_range, src, *offset);
                    if transformed_range.is_some() {
                        //println!("Pushing transformed range: {:?}", transformed_range);
                        output_ranges.push(transformed_range.unwrap());
                        found = true;
                        if remainder_range.is_some() {
                            //println!("Pushing remainder range: {:?}", remainder_range);
                            remainder_ranges.push(remainder_range.unwrap());
                        }
                        break;
                    }
                }
                if !found {
                    //println!("Pushing original range: {:?}", input_range);
                    output_ranges.push(input_range.clone());
                }
                output_ranges.into_iter()
            }).collect();
            //println!("Transformed ranges after: {:?}", transformed_ranges);
            // seed_ranges has no more entries, because the iter consumed all of them
            seed_ranges.clear();
            next_step_ranges.append(&mut transformed_ranges);
            if remainder_ranges.len() > 0 {
                seed_ranges.append(&mut remainder_ranges);
            } else {
                ranges_to_consume = false;
            }
        }
        // This step is done, replace all old ranges with new transformed ones
        seed_ranges.clear();
        seed_map.clear();
        seed_ranges.append(&mut next_step_ranges);

        // Now iterate over the seeds with the current map and transform them all
        // seeds.iter_mut().for_each(|s_id| {
        //     for (range, offset) in seed_map.iter() {
        //         if range.contains(&s_id) {
        //             *s_id = *s_id + offset;
        //             break;
        //         }
        //     }
        //     // let new_sid = seed_map.get(s_id).unwrap_or_else(|| s_id);
        //     // *s_id = *new_sid;
        // });
        //println!("Seeds after = {:?}", seeds);
    }
    // *seeds.iter().min().unwrap()
    *seed_ranges.iter()
        .map(|range| {
            range.start()
        }).min().unwrap()
}

fn main() {
    let fname: String = get_file_name();
    let input_str = std::fs::read_to_string(fname).unwrap();
    let input_vec: Vec<&str> = input_str.trim().split(':').collect();
    let lowest_num = convert_seeds(&input_vec);
    println!("Lowest location number: {}", lowest_num)
}
