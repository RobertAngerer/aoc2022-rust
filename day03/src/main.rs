use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("{}", solve_part_1());
    println!("{}", solve_part_2());
}

fn solve_part_1() -> u32 {
    let input = read_input_for_day();
    let mut sum: u32 = 0;
    for line in input {
        let (split1, split2) = line.split_at(line.len() / 2);
        let mut already_checked_chars = Vec::new();
        for i in split1.chars() {
            if already_checked_chars.contains(&i) {
                continue;
            }
            if split2.contains(i) {
                sum += get_int_of_char(i);
                already_checked_chars.push(i);
            }
        }
    }
    return sum;
}

fn solve_part_2() -> u32 {
    let mut input = read_input_for_day();
    let mut sum = 0;
    let mut temp_vec: Vec<String> = Vec::new();
    let mut reference_line: String;
    for n in 1..=input.len() {
        temp_vec.push(input.pop().unwrap());
        if temp_vec.len() == 3 {
            reference_line = temp_vec.pop().unwrap();
            println!("{:?}", temp_vec);
            println!("{}", reference_line);
            let mut temp: i8 = 0;
            for l in reference_line.chars() {
                for line in &temp_vec {
                    if line.contains(l) {
                        println!("Line = {}, l  = {}", line, l);
                        temp = temp + 1;
                    }
                }
                if temp == 2 {
                    println!("The winner is {}", l);
                    sum += get_int_of_char(l);
                    temp = 0;
                    break
                }
                temp = 0;
            }
            temp_vec.clear();
        }
    }
    return sum;
}

fn get_int_of_char(input: char) -> u32 {
    let value = input as u32;
    if (input as u8).is_ascii_lowercase() {
        return value - 96;
    }
    return value - 64 + 26;
}

fn read_input_for_day() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input03.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip);
            }
        }
    }
    input
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

