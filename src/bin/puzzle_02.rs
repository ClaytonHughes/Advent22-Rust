mod advent;

fn main() {
    let lines = advent::load_lines("02/puzzle.input");
    // Whyy is the type declaration needed here but not in puzzle_01?
    let lines :Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
    let direct_score = calculate_strategy_direct(&lines);
    let result_score = calculate_strategy_result(&lines);

    println!("The strategy_direct guide predicts a score of {}", direct_score);
    println!("The strategy_result guide predicts a score of {}", result_score);
}

fn calculate_strategy_direct(input: &Vec<&str>) -> u32 {
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

fn calculate_strategy_result(input: &Vec<&str>) -> u32 {
    let mut total_score: u32 = 0;

    for line in input {
        let mut parts = line.split_whitespace();
        let (enemy, result) = ( parts.next().unwrap(), parts.next().unwrap() );

        let score = match result {
            "X" => BaseScore::Lose,
            "Y" => BaseScore::Draw,
            "Z" => BaseScore::Win,
            _ => panic!("Invalid result {}", result)
        };

        let enemy_action = match enemy {
            "A" => GameAction::Rock,
            "B" => GameAction::Paper,
            "C" => GameAction::Scissors,
            _ => panic!("Invalid enemy move {}", enemy)
        };

        let base_score = score as u32;
        let bonus_score = bonus_points(&enemy_action, &score);
        total_score += base_score + bonus_score;
    }

    return total_score;
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameAction {
    Rock = 1,
    Paper,
    Scissors
}

#[derive(Debug, Copy, Clone)]
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

fn bonus_points(enemy: &GameAction, result: &BaseScore) -> u32 {
    match result {
        BaseScore::Draw => *enemy as u32,
        BaseScore::Win => {
            if *enemy == GameAction::Scissors {
                GameAction::Rock as u32
            } else {
                *enemy as u32 + 1
            }
        },
        BaseScore::Lose => {
            if *enemy == GameAction::Rock {
                GameAction::Scissors as u32
            } else {
                *enemy as u32 - 1
            }

        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = advent::load_lines("02/test.input");
        let lines = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_strategy_direct(&lines);

        assert_eq!(score, 15);
    }

    #[test]
    fn test_part2() {
        let lines = advent::load_lines("02/test.input");
        let lines : Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_strategy_result(&lines);

        assert_eq!(score, 12);
    }

    #[test]
    fn test_scissors_results() {
        let lines = vec!["C X".to_string(), "C Z".to_string()];
        let lines : Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_strategy_result(&lines);

        // lose to scissors: 0 + 2 (paper)
        // win to scissors = 6 + 1 (rock)
        assert_eq!(score, 9);
    }

    #[test]
    fn test_rock_results() {
        let lines = vec!["A X".to_string(), "A Z".to_string()];
        let lines : Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
        let score = calculate_strategy_result(&lines);

        // lose to rock: 0 + 3 (scissors)
        // win to rock  = 6 + 2 (paper)
        assert_eq!(score, 11);
    }
}

