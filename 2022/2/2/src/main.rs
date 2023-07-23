use std::fs;

enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

enum DesiredResult {
    Lose,
    Draw,
    Win,
}

struct Round {
    desired_outcome: DesiredResult,
    enemy_move: PlayerMove,
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_points(input);

    let result = calculate_rounds(input);
    println!("{}", result);
}

fn parse_points(string: String) -> Vec<Round> {
    string
        .lines()
        .map(|line| {
            let enemy_move = match line.chars().next().unwrap() {
                'A' => PlayerMove::Rock,
                'B' => PlayerMove::Paper,
                'C' => PlayerMove::Scissors,
                _ => panic!(),
            };

            let desired_outcome = match line.chars().nth(2).unwrap() {
                'X' => DesiredResult::Lose,
                'Y' => DesiredResult::Draw,
                'Z' => DesiredResult::Win,
                _ => panic!(),
            };

            Round {
                desired_outcome,
                enemy_move,
            }
        })
        .collect()
}

fn calculate_rounds(rounds: Vec<Round>) -> u32 {
    rounds.iter().map(calculate_round).sum()
}

fn calculate_round(round: &Round) -> u32 {
    let Round {
        enemy_move,
        desired_outcome,
    } = round;

    let r#move = match (enemy_move, desired_outcome) {
        (PlayerMove::Rock, DesiredResult::Lose) => PlayerMove::Scissors,
        (PlayerMove::Rock, DesiredResult::Draw) => PlayerMove::Rock,
        (PlayerMove::Rock, DesiredResult::Win) => PlayerMove::Paper,
        (PlayerMove::Paper, DesiredResult::Lose) => PlayerMove::Rock,
        (PlayerMove::Paper, DesiredResult::Draw) => PlayerMove::Paper,
        (PlayerMove::Paper, DesiredResult::Win) => PlayerMove::Scissors,
        (PlayerMove::Scissors, DesiredResult::Lose) => PlayerMove::Paper,
        (PlayerMove::Scissors, DesiredResult::Draw) => PlayerMove::Scissors,
        (PlayerMove::Scissors, DesiredResult::Win) => PlayerMove::Rock,
    };

    let outcome_points = match desired_outcome {
        DesiredResult::Lose => 0,
        DesiredResult::Draw => 3,
        DesiredResult::Win => 6,
    };

    let move_points = match r#move {
        PlayerMove::Rock => 1,
        PlayerMove::Paper => 2,
        PlayerMove::Scissors => 3,
    };

    outcome_points + move_points
}

#[test]
fn it_should_work() {
    let input = String::from("A Y\nB X\nC Z");

    let input = parse_points(input);

    let result = calculate_rounds(input);

    assert_eq!(result, 12);
}
