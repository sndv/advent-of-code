use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim_end()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

pub fn find_number(numbers: Vec<i64>, target: usize) -> i64 {
    let mut positions = HashMap::new();
    for (i, n) in numbers.iter().take(numbers.len() - 1).enumerate() {
        positions.insert(*n, i + 1);
    }
    let mut curr_number = *(numbers.last().unwrap());
    for i in numbers.len()..target {
        let new_number = match positions.get(&curr_number) {
            Some(pos) => i - pos,
            None => 0,
        };
        positions.insert(curr_number, i);
        curr_number = new_number as i64;
    }
    curr_number
}

pub fn day15a(input: &str) -> i64 {
    let input_p = parse_input(input);
    find_number(input_p, 2020)
}

pub fn day15b(input: &str) -> i64 {
    let input_p = parse_input(input);
    find_number(input_p, 30000000)
}
