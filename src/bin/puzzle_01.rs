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

    let (elf, calories) = largest_elf_calories(&lines);
    println!("{} is carrying {} calories", elf, calories);

    let (elves, calories) = largest_3_elf_calories(&lines);
    println!("Elves {:?} are carrying {} total calories", elves, calories);
}

fn largest_elf_calories(lines: &Vec<String>) -> (usize, u32) {
    let mut elves = Vec::new();
    let mut calories: u32 = 0;
    for line in lines {
        match &line[..] {                                           // I have no idea what is going on with the borrowing here... :(
            "" => {
                elves.push(calories);
                calories = 0;
            }
            _ => {
                let parsed = match line.parse::<u32>() {
                    Err(why) => panic!("Couldn't parse int from {}: {}", line, why),
                    Ok(number) => number,
                };
                calories += parsed;
            }
        }
    }

    let (largest, amount) = elves.iter().enumerate() // gives a tuple (index: usize, value: T)
        .max_by(|(_, a), (_, b)| a.cmp(b)) // method is different than examples on SO. Closure takes two (usize, T) and returns a std::cmp::Ordering (not a bool)
        .unwrap();

    (largest + 1, *amount)  // 1-indexed. I don't know why the dereference here is needed =(
}

fn largest_3_elf_calories(lines: &Vec<String>) -> ([usize; 3], u32) {
    panic!("implement me");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let path = Path::new("./input/01/test.input");
        let file = match File::open(&path) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<_,_>>().unwrap();

        let (elf, calories) = largest_elf_calories(&lines);
        assert_eq!(elf, 4);
        assert_eq!(calories, 24000);
    }

    #[test]
    fn test_part2() {
        let path = Path::new("./input/01/test.input");
        let file = match File::open(&path) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<_,_>>().unwrap();

        let (top3, calories) = largest_3_elf_calories(&lines);
        assert_eq!(top3, [4, 3, 5]);
        assert_eq!(calories, 45000);
    }
}

