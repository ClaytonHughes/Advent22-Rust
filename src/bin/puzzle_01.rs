use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    // I should create a function for this...
    let path = Path::new("./input/01/puzzle.input");
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<_,_>>().unwrap();

    let largest = largest_elf(lines);

    println!("{}", largest);
}

fn largest_elf(lines: Vec<String>) -> usize {
    let mut elves = Vec::new();
    let mut calories: i32 = 0;
    for line in lines {
        match &*line {
            "" => {
                elves.push(calories);
                calories = 0;
            }
            _ => {
                let parsed = match line.parse::<i32>() {
                    Err(why) => panic!("Couldn't parse int from {}: {}", line, why),
                    Ok(number) => number,
                };
                calories += parsed;
            }
        }
    }

    let (largest, _amount) = elves.iter().enumerate() // gives a tuple (index: usize, value: T)
        .max_by(|(_, a), (_, b)| a.cmp(b)) // method is different than examples on SO. Closure takes two (usize, T) and returns a std::cmp::Ordering (not a bool)
        .unwrap();

    largest + 1  // 1-indexed
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_input() {
        let path = Path::new("./input/01/test.input");
        let file = match File::open(&path) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<_,_>>().unwrap();

        assert_eq!(largest_elf(lines), 4);
    }
}

