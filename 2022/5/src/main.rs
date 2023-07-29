use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    println!("part one -> {}", calculate_single_move(&input));
    println!("part two -> {}", calculate_multi_move(&input));
}

#[derive(Debug)]
struct Stack {
    index: usize,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Move {
    from_stack: usize,
    to_stack: usize,
    amount: usize,
}

fn parse_input(input: &str) -> (Vec<Move>, Vec<Stack>) {
    let mut iter = input.split("\n\n");
    let stacks = parse_stacks(iter.next().unwrap());
    let moves = parse_moves(iter.next().unwrap());
    (moves, stacks)
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            Move {
                from_stack: parts[3].parse().unwrap(),
                to_stack: parts[5].parse().unwrap(),
                amount: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let mut iter = input.lines().rev();
    let first_line = iter.next().unwrap();
    let mut stacks: Vec<Stack> = first_line
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|str| str.parse().unwrap())
        .map(|index| Stack {
            index,
            crates: vec![],
        })
        .collect();

    for line in iter {
        let chars: Vec<char> = line.chars().collect();
        let mut crates_new: Vec<Option<char>> = vec![];
        stacks
            .iter()
            .map(|stack| {
                if stack.index == 1 {
                    1
                } else {
                    ((stack.index - 1) * 4) + 1
                }
            })
            .for_each(|index| {
                if chars[index] != ' ' {
                    crates_new.push(Some(chars[index]));
                } else {
                    crates_new.push(None);
                }
            });

        crates_new
            .iter()
            .enumerate()
            .for_each(|(index, char)| match char {
                Some(char) => stacks[index].crates.push(char.to_owned()),
                None => (),
            })
    }
    stacks
}

fn calculate_multi_move(input: &str) -> String {
    let (moves, mut stacks) = parse_input(input);

    moves.iter().for_each(|mv| {
        let index_from = mv.from_stack - 1;
        let index_to = mv.to_stack - 1;

        let index_to_split = stacks[index_from].crates.len() - mv.amount;

        let mut crates_to_move = stacks[index_from].crates.split_off(index_to_split);

        stacks[index_to].crates.append(&mut crates_to_move);
    });

    stacks
        .iter_mut()
        .map(|stack| stack.crates.pop().unwrap())
        .collect::<String>()
}
fn calculate_single_move(input: &str) -> String {
    let (moves, mut stacks) = parse_input(input);

    moves.iter().for_each(|mv| {
        let index_from = mv.from_stack - 1;
        let index_to = mv.to_stack - 1;
        for _i in 0..mv.amount {
            let cr = stacks[index_from].crates.pop().unwrap();
            stacks[index_to].crates.push(cr);
        }
    });

    stacks
        .iter_mut()
        .map(|stack| stack.crates.pop().unwrap())
        .collect::<String>()
}

#[test]
fn should_calc() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let result = calculate_single_move(input);
    assert_eq!(result, "CMZ");
    let result = calculate_multi_move(input);
    assert_eq!(result, "MCD");
}
