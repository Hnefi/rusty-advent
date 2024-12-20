pub mod io_utilities {
    pub fn read_file_to_string(file_name: &String) -> String {
        std::fs::read_to_string(file_name).unwrap()
    }

    pub fn read_file_lines(file_name: &String) -> Vec<String> {
        let file_string = read_file_to_string(file_name);
        file_string.lines().map(|line| line.to_string()).collect()
    }
}

pub mod arg_parsing {
    pub fn get_file_name_or_quit() -> String {
        let args: Vec<String> = std::env::args().collect();
        let arg_len = args.len();
        if arg_len < 2 {
            println!("Provided args didn't have a filename! args = {:?}", args);
            quit::with_code(1);
        }
        args[arg_len - 1].clone()
    }
}
