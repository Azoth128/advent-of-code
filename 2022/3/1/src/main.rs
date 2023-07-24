use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let result = calculate_priorities(input);
    println!("{}", result);
}

struct Backpack {
    compartment_1: String,
    compartment_2: String,
}

fn calculate_priorities(input: String) -> u32 {
    parse_input(input)
        .iter()
        .map(find_duplicate)
        .map(get_priority)
        .sum()
}

fn parse_input(input: String) -> Vec<Backpack> {
    let backpacks = input.lines().map(convert_to_backpack).collect();

    backpacks
}

fn convert_to_backpack(backpack_str: &str) -> Backpack {
    let len = backpack_str.len();

    let (comp1, comp2) = backpack_str.split_at(len / 2);

    Backpack {
        compartment_1: String::from(comp1),
        compartment_2: String::from(comp2),
    }
}

fn find_duplicate(backpack: &Backpack) -> char {
    let comp2 = &backpack.compartment_2;

    backpack
        .compartment_1
        .chars()
        .find(|char| comp2.chars().find(|char2| char == char2).is_some())
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
    assert_eq!(result, 157);
}
