use std::fs::File;
use std::{fs, io};
use std::collections::HashSet;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = read_file();
    solve01(&input);
    solve02(&input);
}

fn solve02(input: &str) {
    let mut first_14 = &input[1534..1548];
    println!("{:?}, {:?}", first_14, first_14.len());
    let mut counter = 1534;
    let start_string = &input[1548..];
    for i in start_string.chars() {
        // println!("{:?}, {:?}", first_14, i);
        if first_14.contains(i) {
            first_14 = &input[counter-12..counter+1];
            counter +=1
        }
        else if contains_duplicates(first_14){
            first_14 = &input[counter-13..counter+1];
            counter +=1
        }
        else {
            println!("{:?}, {:?}", first_14, i);
            println!("{:?}", counter + 1);
            break
        }
    }
}

fn solve01(input: &str) {
    let mut first_three = &input[..3];
    // println!("{:?}", first_three);
    let mut counter = 3;
    let start_string = &input[3..];
    for i in start_string.chars() {
        // println!("{:?}, {:?}", first_three, i);
        if first_three.contains(i) {
            first_three = &input[counter-2..counter+1];
            counter +=1
        }
        else if contains_duplicates(first_three){
            first_three = &input[counter-2..counter+1];
            counter +=1
        }
        else {
            // println!("{:?}, {:?}", first_three, i);
            // println!("{:?}", counter + 1);
            break
        }
    }

}


fn contains_duplicates(input: &str) -> bool {
    let mut set = HashSet::new();
    for c in input.chars() {
        if set.contains(&c) {
            return true;
        }
        set.insert(c);
    }
    false
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
