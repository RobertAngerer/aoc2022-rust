use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input: Vec<String> = read_input_for_day();
    let mut max = 0;
    let mut sum = 0;
    for line in input {
        if line == "" {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue
        }
        sum += line.parse::<i32>().unwrap();
    }
    println!("Day1/1 = {}", max);
    println!("Day1/2 = {}", solve_part_two());
}

fn solve_part_two() -> i32 {
    let input: Vec<String> = read_input_for_day();
    let mut sums: Vec<i32> = Vec::new();
    let mut max = 0;
    let mut sum = 0;
    for line in input {
        if line == "" {
            sums.push(sum);
            sum = 0;
            continue
            }
        sum += line.parse::<i32>().unwrap();
    }
    sums.sort();
    println!("{:?}", sums);
    println!("{:?}", sums[sums.len() - 1]);
    println!("{:?}", sums[sums.len() - 2]);
    println!("{:?}", sums[sums.len() - 3]);

    return sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
}

fn read_input_for_day() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input01.txt") {
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