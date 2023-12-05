use std::{env, collections::HashMap, ops::Range};

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

fn convert_seeds (input: &Vec<&str>) -> i64 {
    // algorithm:
    // - first build the list of seeds (input)
    // - then, for every following "map", build a HashSet containing the mappings of src->dest id
    // - convert the seed list to the next representation, and finally output the minimum

    // Parse the seeds in the first line
    let seed_string: Vec<&str> = input[1].split('\n').collect();
    //println!("Seed string = {:?}",seed_string);
    let mut seeds: Vec<i64> = seed_string.first().unwrap()
        .trim()
        .split(" ")
        .map(|c| c.parse().unwrap())
        .collect();

    // Start from second line where the "maps" begin.
    for idx in 2..input.len() {
        //println!("Map: {:?}", input[idx])
        let mappings: Vec<&str> = input[idx].split('\n').map(|s| s.trim()).collect();
        //println!("Mappings: {:?}", mappings);
        let mut seed_map: HashMap<Range<_>, i64> = HashMap::new();
        mappings.iter().for_each(|m| {
            let range = m.split(" ").collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i64>>();
            if range.len() == 3 { // hack
                let (dst, src, len) = match &range[..] {
                    &[first, second, third, ..] => (first, second, third),
                    _ => unreachable!()
                };
                // Build the source range.
                let source_range = Range {start: src, end: src+len};

                // seed map contains an "offset"
                let offset: i64 = dst - src;
                //println!("Inserting source range w/ start {}, end {}, offset {}", source_range.start, source_range.end, offset);
                seed_map.insert(source_range,offset);
            }
        });

        //println!("Seeds before = {:?}", seeds);
        // Now iterate over the seeds with the current map and transform them all
        seeds.iter_mut().for_each(|s_id| {
            for (range, offset) in seed_map.iter() {
                if range.contains(&s_id) {
                    *s_id = *s_id + offset;
                    break;
                }
            }
            // let new_sid = seed_map.get(s_id).unwrap_or_else(|| s_id);
            // *s_id = *new_sid;
        });
        //println!("Seeds after = {:?}", seeds);
    }
    *seeds.iter().min().unwrap()
}

fn main() {
    let fname: String = get_file_name();
    let input_str = std::fs::read_to_string(fname).unwrap();
    let input_vec: Vec<&str> = input_str.trim().split(':').collect();
    let lowest_num = convert_seeds(&input_vec);
    println!("Lowest location number: {}", lowest_num)
}
