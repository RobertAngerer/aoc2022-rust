use std::env::set_current_dir;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Interval {
    lower_bound: i32,
    upper_bound: i32,
}

impl Interval {
    fn contains_other(&self, second: &Interval) -> bool {
        if self.lower_bound <= second.lower_bound && self.upper_bound >= second.upper_bound {
            return true
        }
        return false
    }
}

impl Interval {
    fn overlap(&self, second: &Interval) -> bool {
        println!("{:?}, {:?}", self, second);
        let mut returnvalue = false;
        if ( self.lower_bound >= second.lower_bound && self.lower_bound <= second.upper_bound ) || ( self.upper_bound >= second.lower_bound && self.upper_bound <= second.upper_bound ) {
            returnvalue = true;
        }
        println!("{}", returnvalue);
        return returnvalue
    }
}


fn main() {
    solve01();
    solve02();
}

fn solve02() -> i32 {
    let input = read_input_for_day();

    let mut counter = 0;
    for line in input {
        let splitted_line: Vec<&str> = line.split(',').collect();
        let first_interval = get_range(&splitted_line[0]);
        let second_interval = get_range(&splitted_line[1]);
        if first_interval.overlap(&second_interval) {
            counter = counter + 1;
            continue
        }
        if second_interval.overlap(&first_interval) {
            counter = counter + 1;
            continue
        }
    }
    println!("{}", counter);
    return counter;
}

fn solve01() -> i32 {
    let input = read_input_for_day();

    let mut counter = 0;
    for line in input {
        let splitted_line: Vec<&str> = line.split(',').collect();
        let first_interval = get_range(&splitted_line[0]);
        let second_interval = get_range(&splitted_line[1]);
        if first_interval.contains_other(&second_interval) {
            counter = counter + 1;
            continue
        }
        if second_interval.contains_other(&first_interval) {
            counter = counter + 1;
            continue
        }
    }
    println!("{}", counter);
    return counter;
}

fn get_range(split_line: &str) -> Interval {
    let split_range: Vec<&str> = split_line.split('-').collect();
    return Interval {
        lower_bound: split_range[0].parse::<i32>().unwrap(),
        upper_bound: split_range[1].parse::<i32>().unwrap(),
    };
}

fn contains_other() -> bool {
    return false;
}

// fn get_range(splitted_line: &str) -> Interval {
//     let splitted_expression: Vec<&str> = splitted_line.split('-').collect();
//     return Interval {
//         lower_bound: splitted_expression[0].parse::<i32>().unwrap(),
//         upper_bound: splitted_expression[1].parse::<i32>().unwrap()
//     }
// }

fn get_parts(input: &str) -> (i32, i32) {
    let parts: Vec<&str> = input.split('-').collect();
    return (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
}


fn read_input_for_day() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input04.txt") {
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
