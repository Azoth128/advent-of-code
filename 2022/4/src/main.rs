use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result_total = calculate_number_total_overlaps(&input);
    let result_partial = calculate_number_partial_overlap(&input);
    println!("total: {}\npartial: {}", result_total, result_partial);
}

#[derive(Debug)]
struct Assignment {
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct Pair {
    elve_1: Assignment,
    elve_2: Assignment,
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            let mut elves = line.split(',');
            let elve_1 = parse_assignment(elves.next().unwrap());
            let elve_2 = parse_assignment(elves.next().unwrap());
            Pair { elve_2, elve_1 }
        })
        .collect()
}
fn parse_assignment(assignment: &str) -> Assignment {
    let mut assignment = assignment.split('-');
    Assignment {
        from: assignment.next().unwrap().parse().unwrap(),
        to: assignment.next().unwrap().parse().unwrap(),
    }
}

fn calculate_number_partial_overlap(input: &str) -> u32 {
    let pairs = parse_input(input);
    pairs
        .iter()
        .map(|pair| {
            let result = ((pair.elve_1.from <= pair.elve_2.to
                && pair.elve_1.from >= pair.elve_2.from)
                || (pair.elve_1.to <= pair.elve_2.to && pair.elve_1.to >= pair.elve_2.from))
                || ((pair.elve_2.from <= pair.elve_1.to && pair.elve_2.from >= pair.elve_1.from)
                    || (pair.elve_2.to <= pair.elve_1.to && pair.elve_2.to >= pair.elve_1.from));

            //println!("{:?} ==> {:?}", pair, result);
            result
        })
        .map(|is_intersecting| if is_intersecting { 1 } else { 0 })
        .sum()
}

fn calculate_number_total_overlaps(input: &str) -> u32 {
    let pairs = parse_input(input);
    pairs
        .iter()
        .map(|pair| {
            (pair.elve_2.from <= pair.elve_1.from && pair.elve_2.to >= pair.elve_1.to)
                || (pair.elve_1.from <= pair.elve_2.from && pair.elve_1.to >= pair.elve_2.to)
        })
        .map(|is_intersecting| if is_intersecting { 1 } else { 0 })
        .sum()
}

#[test]
fn should_be_correct() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let result_total = calculate_number_total_overlaps(input);
    let result_partial = calculate_number_partial_overlap(input);

    assert_eq!(result_total, 2);
    assert_eq!(result_partial, 4);
}
