use std::{env, collections::HashMap};

use regex::Regex;

#[derive(Default)]
struct Network {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
    start_list: Vec<String>
}

impl Network {
    // Use a builder class to create the map
    pub fn builder(fname: String) -> NetworkBuilder {
        NetworkBuilder::new(fname)
    }

    // Functions to get the next node in L and R directions
    // Can we do this without clone? Sounds terrible for runtime.
    pub fn get_left_node(&self, cur_node: &String) -> String {
        self.nodes[cur_node].0.clone()
    }

    pub fn get_right_node(&self, cur_node: &String) -> String {
        self.nodes[cur_node].1.clone()
    }
}

struct NetworkBuilder{
    network_file_name: String
}

impl NetworkBuilder {
    pub fn new(network_fname: String) -> NetworkBuilder {
        NetworkBuilder { network_file_name: network_fname }
    }

    pub fn build(self) -> Network {
        let mut net: Network = Network::default();
        let input = std::fs::read_to_string(self.network_file_name).unwrap();
        let lines: Vec<&str> = input
            .split("\n")
            .collect();


        let mut starting_node_strings: Vec<String> = Vec::new();
        // the first line contains all the directions (e.g., LRLR) so split that into
        // the Network's "directions" field
        net.directions = lines
            .first()
            .unwrap()
            .chars()
            .collect();

        // All lines contain nodes and their adjacencies, so match each one against
        // this pattern, for e.g.,
        //  AAA = (BBB, CCC)
        //  (Node) = (Next-Node-L, Next-Node-R)
        //  (\W+) = (\W+, \W+)
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        //let re = Regex::new(r"([[:alpha]]+) = \(([[:alpha:]]), ([[:alpha:]])\)").unwrap();
        let re = Regex::new(r"([[[:alpha:]]\d]+)\s=\s\x28([[[:alpha:]]\d]+),\s([[[:alpha:]]\d]+)\x29").unwrap();
        lines.iter()
            .for_each(|line| {
                for (_, [node, l_str, r_str]) in re.captures_iter(line).map(|c| c.extract()) {
                    // We know all the keys are unique (tested and implied in the problem statement)
                    map.insert(node.to_string(), (l_str.to_string(), r_str.to_string()));

                    // Put starting nodes on the list
                    if node.ends_with('A') {
                        starting_node_strings.push(node.to_string());
                    }
                }
            });
        net.start_list = starting_node_strings;
        net.nodes = map;
        net
    }
}

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Provided args didn't have a filename! args = {:?}", args);
        quit::with_code(1);
    }
    args[arg_len - 1].clone()
}


fn main() {
    let fname = get_file_name();
    let network: Network = Network::builder(fname).build();

    // Part 1 - start from one node and traverse to the end 'ZZZ'
    // let mut cur_node = "AAA".to_string();
    // let mut found = false;
    // let mut steps = 1;
    // while found == false {
    //     // push all the nodes from the directions list onto the next nodes list
    //     for idx in 0..network.directions.len() {
    //         let next_node = match network.directions[idx] {
    //             'L' => network.get_left_node(&cur_node),
    //             'R' => network.get_right_node(&cur_node),
    //             _ => unreachable!()
    //         };
    //         if next_node == "ZZZ" {
    //             println!("Escaped in {} steps!", steps);
    //             found = true;
    //             break;
    //         }
    //         steps += 1;
    //         cur_node = next_node;
    //     }
    // }

    fn get_next_node(direction_idx: usize, cur_node: &String, network: &Network) -> String {
        let next_node = match network.directions[direction_idx] {
            'L' => network.get_left_node(&cur_node),
            'R' => network.get_right_node(&cur_node),
            _ => unreachable!()
        };
        next_node
    }

    // Part 2 - Start from all nodes ending in A, and go to all those ending in Z.
    // Algorithm:
    // - create two sets: one with the input nodes, one with the output
    // - map each input node to an output
    // - check for termination
    // - swap the two set references, so the output becomes the input of the next step
    let mut found = false;
    let mut steps = 0;
    let mut direction_idx: usize = 0;
    // could optimize this to not clone the start list and strings all the time
    let mut input_nodes = network.start_list.clone();
    let mut output_nodes: Vec<String> = Vec::new();
    let mut ptr_1 = &mut input_nodes;
    let mut ptr_2 = &mut output_nodes;
    while found == false {
        //println!("Step {} starting with nodes {:?}...", steps, ptr_1);
        ptr_1.into_iter().for_each(|node| {
            ptr_2.push(get_next_node(direction_idx, &node, &network));
        });
        //println!("Transformed into nodes {:?}...", ptr_2);

        // check termination
        found = ptr_2.iter().all(|node| {
            if node.ends_with('Z') {
                //println!("Found that at least 1 node ends with Z! Step {}. Nodes = {:?}", steps, ptr_2);
                return true;
            } else {
                return false;
            }
        });
        ptr_1.clear();
        // swap references and reset direction_idx for the next iteration
        let tmp = ptr_1;
        ptr_1 = ptr_2;
        ptr_2 = tmp;
        direction_idx += 1;
        if direction_idx >= network.directions.len() {
            direction_idx = 0;
        }
        steps += 1;
    }
    println!("Escaped in {} steps!", steps);
}