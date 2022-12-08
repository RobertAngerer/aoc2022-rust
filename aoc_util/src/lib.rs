use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod aoc_util {
    fn read_input_for_day() {}
}

pub fn read_input_for_day(day: String) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input".to_owned() + &day + ".txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip);
            }
        }
    }
    input
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





