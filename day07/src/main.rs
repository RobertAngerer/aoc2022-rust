use std::{fs, io};
use std::fs::File;
use std::path::Path;

fn main() {
    println!("Hello, world!");
}
fn read_file() -> String {
    let contents = fs::read_to_string("input06.txt").expect("should");
    // println!("With text:\n{contents}");
    return contents
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
