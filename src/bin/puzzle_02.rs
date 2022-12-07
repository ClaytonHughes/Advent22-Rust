mod advent;

fn main() {
    let lines = advent::load_lines("02/puzzle.input");
    // Whyy is the type declaration needed here but not in puzzle_01?
    let lines :Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
    let score = calculate_strategy(&lines);

    println!("The strategy guide predicts a score of {}", score);
}

fn calculate_strategy(input: &Vec<&str>) -> u32 {
    let mut total_score: u32 = 0;

    for line in input {
        let mut parts = line.split_whitespace();
        let (enemy, me) = ( parts.next().unwrap(), parts.next().unwrap() );

        let my_action = match me {
            "X" => GameAction::Rock,
            "Y" => GameAction::Paper,
            "Z" => GameAction::Scissors,
            _ => panic!("Invalid my move {}", me)
        };

        let enemy_action = match enemy {
            "A" => GameAction::Rock,
            "B" => GameAction::Paper,
            "C" => GameAction::Scissors,
            _ => panic!("Invalid enemy move {}", enemy)
        };

        let base_score = base_points(&my_action, &enemy_action);
        let bonus_score = my_action as u32;
        total_score += base_score + bonus_score;
    }

    return total_score;
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GameAction {
    Rock = 1,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum BaseScore {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn base_points(me: &GameAction, enemy: &GameAction) -> u32 {
    if me == enemy {
        return BaseScore::Draw as u32;
    }

    if (*me == GameAction::Rock && *enemy == GameAction::Paper) ||
        (*me == GameAction::Paper && *enemy == GameAction::Scissors) ||
        (*me == GameAction::Scissors && *enemy == GameAction::Rock)
    {
        return BaseScore::Lose as u32;
    }

    return BaseScore::Win as u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("02/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_strategy(&lines);

        println!("{}", score);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_part2() {
        let lines = advent::load_lines("02/test.input");
        let lines : Vec<&str> = lines.iter().map(|l| l.as_str()).collect();


    }
}

