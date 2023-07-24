use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let result = calculate_priorities(input);
    println!("{}", result);
}

struct Backpacks {
    elve_1: String,
    elve_2: String,
    elve_3: String,
}

fn calculate_priorities(input: String) -> u32 {
    parse_input(input)
        .iter()
        .map(find_duplicate)
        .map(get_priority)
        .sum()
}

fn parse_input(input: String) -> Vec<Backpacks> {
    let mut elves = vec![];
    let mut backpacks = vec![];

    for line in input.lines() {
        elves.push(line);
        if elves.len() == 3 {
            let backpack = convert_to_backpack(&elves);
            backpacks.push(backpack);
            elves.clear();
        }
    }
    backpacks
}

fn convert_to_backpack(elves: &Vec<&str>) -> Backpacks {
    Backpacks {
        elve_1: String::from(elves[0]),
        elve_2: String::from(elves[1]),
        elve_3: String::from(elves[2]),
    }
}

fn find_duplicate(backpacks: &Backpacks) -> char {
    backpacks
        .elve_1
        .chars()
        .find(|char1| {
            backpacks
                .elve_2
                .chars()
                .find(|char2| char1 == char2)
                .is_some()
                && backpacks
                    .elve_3
                    .chars()
                    .find(|char3| char1 == char3)
                    .is_some()
        })
        .unwrap()
}

fn get_priority(item: char) -> u32 {
    let mut priority = item as u32;

    if priority > 96 {
        priority -= 96
    } else {
        priority -= 38
    }

    priority
}

#[test]
fn it_should_calculate() {
    let input = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
    );

    let result = calculate_priorities(input);
    assert_eq!(result, 70);
}
