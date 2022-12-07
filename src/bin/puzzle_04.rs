mod advent;

use itertools::Itertools;

fn main() {
    let lines = advent::load_lines("04/puzzle.input");
    let lines = lines.iter().map(|s| s.as_str()).collect();

    let subsets = count_subsets(&lines);
    println!("There are {} section subsets in the assignments", subsets);
    let overlaps = count_overlaps(&lines);
    println!("There are {} section overlaps in the assignments", overlaps);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SectionAssignment {
    start: u32,
    end: u32,
}

impl SectionAssignment {
    pub fn new(definition: &str) -> Self {
        let bounds = definition.split('-').collect::<Vec<&str>>();
        let (start, end) = bounds.iter().next_tuple().unwrap();

        SectionAssignment {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap()
        }
    }
}

fn count_subsets(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for line in lines {
        let pair = line.split(',').collect::<Vec<&str>>();
        let (adef, bdef) = pair.iter().next_tuple().unwrap();
        let a = SectionAssignment::new(adef);
        let b = SectionAssignment::new(bdef);
        if (a.start <= b.start && b.end <= a.end) ||
           (b.start <= a.start && a.end <= b.end)
        {
            total += 1;
        }
    }

    return total;
}

fn count_overlaps(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for line in lines {
        let pair = line.split(',').collect::<Vec<&str>>();
        let (adef, bdef) = pair.iter().next_tuple().unwrap();
        let a = SectionAssignment::new(adef);
        let b = SectionAssignment::new(bdef);
        if (a.start <= b.start && b.start <= a.end) ||
           (b.start <= a.start && a.start <= b.end)
        {
            total += 1;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("04/test.input");
        let lines = lines.iter().map(|s| s.as_str()).collect();

        let subsets = count_subsets(&lines);
        assert_eq!(subsets, 2);
    }

    #[test]
    fn test_struct_ctor() {
        let wide = SectionAssignment::new("42-1234567890");

        assert_eq!(wide.start, 42);
        assert_eq!(wide.end, 1234567890);
    }

    #[test]
    fn test_part2() {
        let lines = advent::load_lines("04/test.input");
        let lines = lines.iter().map(|s| s.as_str()).collect();

        let subsets = count_overlaps(&lines);
        assert_eq!(subsets, 4);
    }

}
