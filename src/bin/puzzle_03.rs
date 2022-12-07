mod advent;

fn main() {
    let lines = advent::load_lines("03/puzzle.input");
    let lines = lines.iter().map(|l| l.as_str()).collect();
    let score = calculate_priorities(&lines);

    println!("Total priority of mispacked items is {}", score);
}

fn calculate_priorities(lines: &Vec<&str>) -> u32 {
    return 0;
}

fn split_rucksack(contents: &str) -> (String, String) {
    let len = contents.len() >> 1; // we don't have to worry about non-ascii characters
    let left = contents[0usize..len].to_string();
    let right = contents[len..].to_string();

    return (left, right);
}

fn find_duplicate(left: &str, right: &str) -> char {
    return 'a';
}

fn sum_priorities(dupes: &Vec<char>) -> u32 {
    return 0;
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
    }

    #[test]
    fn has_correct_priorities() {
        let dupes :Vec<char> = "a".chars().collect();
        let score = sum_priorities(&dupes);
        assert_eq!(score, 1);

        let dupes = "z".chars().collect();
        let score = sum_priorities(&dupes);
        assert_eq!(score, 26);

        let dupes = "A".chars().collect();
        let score = sum_priorities(&dupes);
        assert_eq!(score, 27);

        let dupes = "A".chars().collect();
        let score = sum_priorities(&dupes);
        assert_eq!(score, 52);
    }

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("03/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_priorities(&lines);

        assert_eq!(score, 157)
    }
}
