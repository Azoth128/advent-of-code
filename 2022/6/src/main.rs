use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    println!(
        "package: {}\nmessage: {}",
        calculate_start_of_package(&input),
        calculate_start_of_message(&input)
    );
}

fn calculate_start_of_package(signal: &str) -> usize {
    let mut last_4_chars = VecDeque::new();
    for (i, c) in signal.chars().enumerate() {
        last_4_chars.push_back(c);
        if last_4_chars.len() > 4 {
            let _ = last_4_chars.pop_front();
        }

        if last_4_chars.len() < 4 {
            continue;
        }

        if contains_duplicates(last_4_chars.clone().into()) {
            continue;
        }

        return i + 1;
    }
    panic!();
}
fn calculate_start_of_message(signal: &str) -> usize {
    let mut last_14_chars = VecDeque::new();
    for (i, c) in signal.chars().enumerate() {
        last_14_chars.push_back(c);
        if last_14_chars.len() > 14 {
            let _ = last_14_chars.pop_front();
        }

        if last_14_chars.len() < 14 {
            continue;
        }

        if contains_duplicates(last_14_chars.clone().into()) {
            continue;
        }

        return i + 1;
    }
    panic!();
}

fn contains_duplicates(chars: Vec<char>) -> bool {
    for i in 0..chars.len() {
        let char1 = chars[i];
        for j in (i + 1)..chars.len() {
            let char2 = chars[j];
            if char1 == char2 {
                return true;
            }
        }
    }

    false
}

#[test]
fn should_calculate() {
    assert_eq!(
        calculate_start_of_package("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
        7
    );
    assert_eq!(
        calculate_start_of_package("bvwbjplbgvbhsrlpgdmjqwftvncz"),
        5
    );
    assert_eq!(
        calculate_start_of_package("nppdvjthqldpwncqszvftbrmjlhg"),
        6
    );
    assert_eq!(
        calculate_start_of_package("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        10
    );
    assert_eq!(
        calculate_start_of_package("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        11
    );

    assert_eq!(
        calculate_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
        19
    );
    assert_eq!(
        calculate_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"),
        23
    );
    assert_eq!(
        calculate_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"),
        23
    );
    assert_eq!(
        calculate_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        29
    );
    assert_eq!(
        calculate_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        26
    );
}
