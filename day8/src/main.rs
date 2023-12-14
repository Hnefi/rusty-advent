use std::{env, collections::HashMap};

use regex::Regex;

#[derive(Default)]
struct Network {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
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
        let re = Regex::new(r"([[:alpha:]]+)\s=\s\x28([[:alpha:]]+),\s([[:alpha:]]+)\x29").unwrap();
        lines.iter()
            .for_each(|line| {
                for (_, [node, l_str, r_str]) in re.captures_iter(line).map(|c| c.extract()) {
                    // We know all the keys are unique (tested and implied in the problem statement)
                    map.insert(node.to_string(), (l_str.to_string(), r_str.to_string()));
                }
            });
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

    //let mut next_nodes: Vec<&String> = Vec::new();
    let mut cur_node = "AAA".to_string();
    let mut found = false;
    let mut steps = 1;
    while found == false {
        // push all the nodes from the directions list onto the next nodes list
        for idx in 0..network.directions.len() {
            let next_node = match network.directions[idx] {
                'L' => network.get_left_node(&cur_node),
                'R' => network.get_right_node(&cur_node),
                _ => unreachable!()
            };
            if next_node == "ZZZ" {
                println!("Escaped in {} steps!", steps);
                found = true;
                break;
            }
            steps += 1;
            cur_node = next_node;
        }
    }
}
