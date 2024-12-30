fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len != 3 {
        println!("Incorrect number of provided args = {:?}", args);
        return;
    }
    let page_outputs = args[arg_len - 1].clone();
    let rules = args[arg_len - 2].clone();
    println!(
        "Hello from AOC Day 5! Parsing puzzle input rules: {rules}, and also parsing page outputs: {page_outputs}",
    );
}
