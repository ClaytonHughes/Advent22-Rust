mod advent;

use std::collections::HashSet;

fn main() {
    let lines = advent::load_lines("03/puzzle.input");
    let lines = lines.iter().map(|l| l.as_str()).collect();
    let score = calculate_priorities(&lines);
    let badge_score = calculate_group_priorities(&lines);

    println!("Total priority of mispacked items is {}", score);
    println!("Total priority of group badges is {}", badge_score);
}

fn calculate_priorities(lines: &Vec<&str>) -> u32 {
    let mut total_priority = 0;
    for line in lines {
        let (left, right) = split_rucksack(line);
        let dupe = find_duplicate(left.as_str(), right.as_str());
        let priority = get_priority(dupe);
        total_priority += priority;
    }

    return total_priority;
}

fn calculate_group_priorities(lines: &Vec<&str>) -> u32 {
    return 0;
}

fn split_rucksack(contents: &str) -> (String, String) {
    let len = contents.len() >> 1; // we don't have to worry about non-ascii characters
    let left = contents[0usize..len].to_string();
    let right = contents[len..].to_string();

    return (left, right);
}

fn find_duplicate(left: &str, right: &str) -> char {
    // This was extremely painful to write.
    // I bashed by head against the compiler quite a bit using the
    // "keep changing things until it works" school of developement.
    let lchars: Vec<char> = left.chars().collect();
    let rchars: Vec<char> = right.chars().collect();
    let lset: HashSet<char> = HashSet::from_iter(lchars.into_iter());
    let rset: HashSet<char> = HashSet::from_iter(rchars.into_iter());
    let dupe: Vec<char> = lset.intersection(&rset).map(|i| *i).collect();

    return dupe[0];
}

fn find_badge(a: &str, b: &str, c: &str) -> char {
    return 'a';
}

fn get_priority(dupe: char) -> u32 {
    if 'a' <= dupe && dupe <= 'z' {
        return dupe as u32 - 'a' as u32 + 1;
    }
    return dupe as u32 - 'A' as u32 + 27;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_correctly() {
        let (left, right) = split_rucksack("abcdxyzw");
        assert_eq!(left, "abcd");
        assert_eq!(right, "xyzw");
    }

    #[test]
    fn identifies_duplicates() {
        let left = "abcde";
        let right = "xyzwe";
        let dupe = find_duplicate(left, right);
        assert_eq!(dupe, 'e');

        let dupe = find_duplicate_from_scratch(left, right);
        assert_eq!(dupe, 'e');
    }

    #[test]
    fn has_correct_priorities() {

        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("03/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_priorities(&lines);

        assert_eq!(score, 157);
    }

    fn finds_group_badge() {
        let one = "Abcdef";
        let two = "aBCDZA";
        let three = "lmnopqrstuvwAxyz";

        let badge = find_badge(one, two, three);

        assert_eq!(badge, 'A');
    }

    #[test]
    fn test_part2() {
        let lines = advent::load_lines("03/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_group_priorities(&lines);

        assert_eq!(score, 70);
    }
}

// this took way less time to write than the one above =(
#[allow(dead_code)]
fn find_duplicate_from_scratch(left: &str, right: &str) -> char {
    let mut array: [bool; 128] = [false; 128];
    for c in left.chars() {
        array[c as usize] = true;
    }
    for c in right.chars() {
        if array[c as usize] == true {
            return c;
        }
    }
    panic!("no duplicate chars");
}
