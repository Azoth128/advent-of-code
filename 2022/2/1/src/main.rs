use std::fs;

enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    me: PlayerMove,
    enemy: PlayerMove,
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
            let enemy = match line.chars().next().unwrap() {
                'A' => PlayerMove::Rock,
                'B' => PlayerMove::Paper,
                'C' => PlayerMove::Scissors,
                _ => panic!(),
            };

            let me = match line.chars().nth(2).unwrap() {
                'X' => PlayerMove::Rock,
                'Y' => PlayerMove::Paper,
                'Z' => PlayerMove::Scissors,
                _ => panic!(),
            };

            Round { me, enemy }
        })
        .collect()
}

fn calculate_rounds(rounds: Vec<Round>) -> u32 {
    rounds.iter().map(calculate_round).sum()
}

fn calculate_round(round: &Round) -> u32 {
    let Round { me, enemy } = round;
    let round_outcome_points = match (me, enemy) {
        (PlayerMove::Rock, PlayerMove::Paper)
        | (PlayerMove::Paper, PlayerMove::Scissors)
        | (PlayerMove::Scissors, PlayerMove::Rock) => 0,

        (PlayerMove::Scissors, PlayerMove::Scissors)
        | (PlayerMove::Rock, PlayerMove::Rock)
        | (PlayerMove::Paper, PlayerMove::Paper) => 3,

        (PlayerMove::Rock, PlayerMove::Scissors)
        | (PlayerMove::Paper, PlayerMove::Rock)
        | (PlayerMove::Scissors, PlayerMove::Paper) => 6,
    };

    let move_points = match me {
        PlayerMove::Rock => 1,
        PlayerMove::Paper => 2,
        PlayerMove::Scissors => 3,
    };

    round_outcome_points + move_points
}

#[test]
fn it_should_work() {
    let input = String::from("A Y\nB X\nC Z");

    let input = parse_points(input);

    let result = calculate_rounds(input);

    assert_eq!(result, 15);
}
