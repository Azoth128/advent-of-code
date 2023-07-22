use std::fs;

enum Content {
    Number(u32),
    None,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = parse_strings(input);

    let mut elves: Vec<u32> = vec![];
    let mut current_elve: u32 = 0;

    for value in input {
        match value {
            Content::Number(i) => current_elve += i,
            Content::None => {
                elves.push(current_elve);
                current_elve = 0;
            }
        };
    }

    println!("highest 1 {}", calculate_highest_values(1, &elves));
    println!("highest 3 {}", calculate_highest_values(3, &elves));
}

fn calculate_highest_values(number_of_elves: u8, elves: &Vec<u32>) -> u32 {
    let mut elves = elves.clone();

    elves.sort_unstable();
    elves.reverse();

    let mut sum = 0;
    for index in 0..number_of_elves {
        sum += elves[index as usize];
    }

    sum
}

fn parse_strings(string: String) -> Vec<Content> {
    string
        .lines()
        .map(|line| {
            if line.is_empty() {
                Content::None
            } else {
                Content::Number(line.parse().unwrap())
            }
        })
        .collect()
}
