use std::{collections::HashMap, env, ops::Range};
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

fn range_size(r: &Range<i64>) -> i64 {
    r.end - r.start + 1
}

fn print_result(v: &Vec<Range<i64>>)  {
    println!("Added new output range, output now is {:?}", v);
}

// Return a vector containing the overlap of two ranges, transformed by the offset, plus the remainder
// range, if any.
fn overlap_and_exclusive(input: &Range<i64>, source: &Range<i64>, offset: i64) -> Vec<Range<i64>> {
    let mut result = Vec::new();
    if source.contains(&input.start) && source.contains(&input.end) {
        println!("Input range {:?} completely contained in source {:?}", input, source);
        // .... S1 -------------- Sn ....
        // ........ I1............In ....
        // .........O1............On .....
        let overlap_start = max(source.start, input.start);
        let overlap_end = min(source.end, input.end);
        result.push(Range {
            start: overlap_start,
            end: overlap_end
        });
        print_result(&result);
        if overlap_start > source.start {
            println!("There is a remainder range from {} to {}", source.start, overlap_start-1);
            // there is a remainder range from source.start to overlap_start-1
            result.push(Range { start: source.start, end: overlap_start-1});
            print_result(&result);
        } else if overlap_end < source.end {
            println!("There is a remainder range from {} to {}", overlap_end+1, source.end);
            result.push(Range { start: overlap_end+1, end: source.end});
            print_result(&result);
        }
    } else if input.start > source.start {
        // potential overlap looks like this:
        // ....Is--Ie...
        // ......Ss....Ee
        // ....Ss--Ie...
        // ......Ss....Ee
        
    } else {

    }
    result
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

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    (0..seed_seeds.len() - 2).for_each(|idx| {
        seed_ranges.push(Range{start: seed_seeds[2 * idx], end: seed_seeds[2*idx] + seed_seeds[2 * idx + 1]});
    });
    println!("Seed ranges: {:?}", seed_ranges);

    //
    // Algorithm:
    // - for each "mapping" of seeds in the problem input set, create two other sets of ranges
    //   like this: {[begin, end], offset}
    //   e.g., {[98, 99], -48}, {[50, 97], 2}
    let mut seed_map: HashMap<Range<i64>, i64> = HashMap::new();
    for idx in 2..input.len() {
        //println!("Map: {:?}", input[idx])
        let mappings: Vec<&str> = input[idx].split('\n').map(|s| s.trim()).collect();
        println!("Mappings: {:?}", mappings);
        mappings.iter().for_each(|m| {
            let range = m
                .split(" ")
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
                let source_range = Range {
                    start: src,
                    end: src + len,
                };

                // seed map contains an "offset"
                let offset: i64 = dst - src;
                println!(
                    "Inserting source range w/ start {}, end {}, with an offset of {}",
                    source_range.start, source_range.end, offset,
                );
                seed_map.insert(source_range, offset);
            }
        });

        // - then, map the input seed ranges to a set of new ranges.
        // - given input [90, 99], and an input of {[98, 99], -48}, {[50, 97], 2}
        //   we would want the output as [92, 99], [50, 51]
        println!("Seed ranges before: {:?}", seed_ranges);
        let new_ranges: Vec<Range<i64>> = seed_ranges.iter().flat_map(|input_range| {
            // return the range overlap between the src and the current range, and then
            // translate it to the destination
            let mut output_ranges: Vec<Range<i64>> = Vec::new();
            for (src, offset) in seed_map.iter() {
                output_ranges.extend(overlap_and_exclusive(input_range, src, *offset));
            }
            output_ranges.into_iter()
        }).collect();
        println!("Seed ranges after: {:?}", new_ranges);

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
    0
}

fn main() {
    let fname: String = get_file_name();
    let input_str = std::fs::read_to_string(fname).unwrap();
    let input_vec: Vec<&str> = input_str.trim().split(':').collect();
    let lowest_num = convert_seeds(&input_vec);
    println!("Lowest location number: {}", lowest_num)
}
