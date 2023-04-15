use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching For \"{:}\"", query);
    println!("In the File {:}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong with the file");
    println!("With the contents \n{}", contents);
}
