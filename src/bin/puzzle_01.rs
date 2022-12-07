mod advent;

struct Elf {
    elf: usize,
    calories: u32,
}

impl Elf {

    fn init() -> Elf {
        Elf { elf: 0, calories: 0}
    }

    fn new(index: usize, calories: u32) -> Elf {
        Elf { elf: index, calories: calories }
    }
}

fn largest_elf_calories(lines: &Vec<&str>) -> (usize, u32) {
    let mut elves = Vec::new();
    let mut calories: u32 = 0;
    for line in lines {
        match *line {
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
//         .max_by(|(_, a), (_, b)| a.cmp(b)) // method is different than examples on SO. Closure takes two (usize, T) and returns a std::cmp::Ordering (not a bool)
        .max_by_key(|&(_, value)| value)    // oh this is almost certainly what I saw on SO and didn't notice the `_key` in the method name.
        .unwrap();

    (largest + 1, *amount)  // 1-indexed. I don't know why the dereference here is needed =(
}

fn largest_3_elf_calories(lines: &Vec<&str>) -> ([usize; 3], u32) {
    let mut elves = Vec::new();
    let mut calories: u32 = 0;
    for line in lines {
        match *line {
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

    // I know this is ridiculous, but I don't know the language and it solves the problem
    let mut largest = Elf::init();
    let mut largest_2 = Elf::init();
    let mut largest_3 = Elf::init();

    for (elf, &calories) in elves.iter().enumerate() { // ... I don't know why the (&) borrow(?) is needed here.
        if largest.calories < calories {    // oh no! my c-style if parens
            largest_3 = largest_2;
            largest_2 = largest;
            largest = Elf::new(elf, calories);
        } else if largest_2.calories < calories {
            largest_3 = largest_2;
            largest_2 = Elf::new(elf, calories);
        } else if largest_3.calories < calories {
            largest_3 = Elf::new(elf, calories);
        }
    }
    // ... I'm a doofus: I could've just sorted it.

    ([largest.elf + 1, largest_2.elf + 1, largest_3.elf + 1], // 1-indexed.
    largest.calories + largest_2.calories + largest_3.calories)
}

fn main() {
    // Shadowing the owned strings seems like the best way to only deal
    // with `&str`s, since I want this to be read only and the `match`
    // statements were a pain with `String`s and the empty string.
    //   There might be a better way to detect that, though...
    let lines = advent::load_lines("01/puzzle.input");
    // Also, I should figure out the differences between types of `iter()`
    let lines = lines.iter().map(|l| l.as_str()).collect();

    let (elf, calories) = largest_elf_calories(&lines);
    println!("Elf {} is carrying {} calories", elf, calories);

    let (elves, calories) = largest_3_elf_calories(&lines);
    println!("Elves {:?} are carrying {} total calories", elves, calories);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("01/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();

        let (elf, calories) = largest_elf_calories(&lines);
        assert_eq!(elf, 4);
        assert_eq!(calories, 24000);
    }

    #[test]
    fn test_part2() {
        let lines = advent::load_lines("01/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();

        let (top3, calories) = largest_3_elf_calories(&lines);
        assert_eq!(top3, [4, 3, 5]);
        assert_eq!(calories, 45000);
    }
}

