use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    solve02();

}


fn solve02() {
    let (input, stack_information) = read_input_for_day_05();
    let mut stacks = setup_stacks_state(stack_information);
    move_crates02(&input, &mut stacks);
    println!("{:?}", stacks);
    let mut output = String::from("");
    for stack in stacks {
        output += &stack.get(stack.len()-1).unwrap().to_string();
    }
    println!("{}", output)
}

fn solve01() {
    let (input, stack_information) = read_input_for_day_05();
    let mut stacks = setup_stacks_state(stack_information);
    move_crates(&input, &mut stacks);
    println!("{:?}", stacks);
    let mut output = String::from("");
    for stack in stacks {
        output += &stack.get(stack.len()-1).unwrap().to_string();
    }
    println!("{}", output)
}

fn move_crates02(input: &Vec<String>, mut stacks: &mut Vec<Vec<char>>) {
    for line in input {
        let (amount, from, to) =  parse_stack_movement_line(line);
        move_crate02(from - 1, to - 1, amount, &mut stacks);
    }
}

fn move_crates(input: &Vec<String>, mut stacks: &mut Vec<Vec<char>>) {
    for line in input {
        let (amount, from, to) =  parse_stack_movement_line(line);
        for i in 0..amount {
            move_crate(from - 1, to - 1, &mut stacks);
        }
    }
}

fn move_crate(from: u32, to: u32, stacks: &mut Vec<Vec<char>>) {
    let from = from as usize;
    let to = to as usize;
    let stack = stacks[from].pop().unwrap();
    stacks[to].push(stack);
}

fn move_crate02(from: u32, to: u32, amount: u32, stacks: &mut Vec<Vec<char>>) {
    println!("inside");
    let from = from as usize;
    let to = to as usize;
    let mut crates_to_move: Vec<char> = Vec::new();
    println!("from = {:?}", stacks[from]);
    println!("to = {:?}", stacks[to]);
    for i in 0..amount {
        println!("{:?}, {:?}, {:?}", crates_to_move, amount, i);
        let stack = stacks[from].pop().unwrap();
        crates_to_move.push(stack);
    }
    println!("crates to move = {:?}", crates_to_move);

    for j in 0..amount {
        let stacki = crates_to_move.pop().unwrap();
        stacks[to].push(stacki)
    }
    println!("to = {:?}", stacks[to]);

}

fn parse_stack_movement_line(line: &str) -> (u32, u32, u32) {
    let split_line: Vec<&str> = line.split(" ").collect();
    return (split_line[1].parse::<u32>().unwrap(), split_line[3].parse::<u32>().unwrap(), split_line[5].parse::<u32>().unwrap())
}

fn setup_stacks_state(stack_information: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for n in 1..10 {
        stacks.push(Vec::new());
    }
    for line in stack_information {
        for i in 0..line.len() {
            if i % 4 == 1 {
                if line.as_bytes()[i] as char != ' ' {
                    stacks.get_mut(i / 4).unwrap().insert(0,line.as_bytes()[i] as char)
                }
            }
        }
    }
    stacks
}
//                         [Z] [W] [Z]
//         [D] [M]         [L] [P] [G]
//     [S] [N] [R]         [S] [F] [N]
//     [N] [J] [W]     [J] [F] [D] [F]
// [N] [H] [G] [J]     [H] [Q] [H] [P]
// [V] [J] [T] [F] [H] [Z] [R] [L] [M]
// [C] [M] [C] [D] [F] [T] [P] [S] [S]
// [S] [Z] [M] [T] [P] [C] [D] [C] [D]
//  1   2   3   4   5   6   7   8   9
fn read_input_for_day_05() -> ( Vec<String>, Vec<String> ) {
    let mut stack_information: Vec<String> = Vec::new();
    let mut input: Vec<String> = Vec::new();
    let mut stack:bool = true;
    if let Ok(lines) = read_lines("input05.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    stack = false;
                    continue
                }
                if ip == " 1   2   3   4   5   6   7   8   9 " {
                continue
                }
                if stack {
                    stack_information.push(ip);
                }
                else {
                    input.push(ip);
                }
            }
        }
    }
    return (input, stack_information)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
