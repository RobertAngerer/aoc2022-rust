use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("{}", solve01());
    println!("{}", solve02())
}

fn solve02() -> i32 {
    let input = read_input_for_day();
    let mut sum: i32 = 0;
    for line in input {
        let first: char = line.chars().nth(0).unwrap();
        let second: char = line.chars().nth(2).unwrap();
        let game_input: char = choose_game_input(second, first);
        println!("Choice = {}", game_input);
        sum += won_game(line.chars().nth(0).unwrap(), choose_game_input(second, first));
    }
    return sum
}

fn solve01() -> i32 {
    let input = read_input_for_day();
    let mut sum: i32 = 0;
    for line in input {
        sum += won_game(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
    }
    return sum
}

fn choose_game_input(choice: char, opponent_input: char) -> char {
    if choice == 'X' {
        if opponent_input == 'A' {
            return 'Z'
        }
        if opponent_input == 'B' {
            return 'X'
        }
        if opponent_input == 'C' {
            return 'Y'
        }
    }
    if choice == 'Y' {
        if opponent_input == 'A' {
            return 'X'
        }
        if opponent_input == 'B' {
            return 'Y'
        }
        if opponent_input == 'C' {
            return 'Z'
        }
    }
    if choice == 'Z' {
        if opponent_input == 'A' {
            return 'Y'
        }
        if opponent_input == 'B' {
            return 'Z'
        }
        if opponent_input == 'C' {
            return 'X'
        }
    }
    return 'A'
}



fn read_input_for_day() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input02.txt") {
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


fn won_game(first: char, second: char) -> i32 {
    let input: char = convert_xyz_to_abc(second);
    let mut score: i32 = 0;
    if first == input {
        return 3 + eval_input_score(second)
    }
    if first == 'A' {
        if input == 'B' {
             score += 6;
        }
        else {
             score += 0;
        }
    }
    if first == 'B' {
        if input == 'A' {
            score += 0;
        }
        else {
            score += 6;
        }
    }
    if first == 'C' {
        if input == 'A' {
            score += 6;
        }
        else {
            score += 0;
        }
    }
    println!("First = {}, Second = {}, outcome = {}", first, input, score + eval_input_score(second));
    return score + eval_input_score(second)
}

fn convert_xyz_to_abc(input: char) -> char {
    if input == 'X' {
        return 'A'
    }
    if input == 'Y' {
        return 'B'
    }
    return 'C'
}

fn eval_input_score(input: char) -> i32 {
    if input == 'X' {
        return 1
    }
    if input == 'Y' {
        return 2
    }
    return 3
}
