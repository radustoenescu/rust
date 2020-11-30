use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    let contents = fs::read_to_string(filename).expect("Unable to read file");

    println!("{}", contents);
}
