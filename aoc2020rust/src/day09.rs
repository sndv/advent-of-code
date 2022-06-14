fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim_end()
        .split("\n")
        .map(|el| el.parse::<i64>().unwrap())
        .collect()
}

fn find_invalid_number(parsed_input: &Vec<i64>, preamble_size: usize) -> i64 {
    let mut previous: Vec<i64> =
        Vec::from_iter(parsed_input[0..preamble_size].iter().map(|el| *el));
    let rest = &parsed_input[preamble_size..];

    *(rest
        .iter()
        .find(|el| {
            let no_sum = previous
                .iter()
                .find(|prev_el| **el != 2 * (**prev_el) && previous.contains(&(*el - *prev_el)))
                .is_none();
            previous.remove(0);
            previous.push(**el);
            no_sum
        })
        .unwrap())
}

pub fn day09a(input: &str, preamble_size: usize) -> i64 {
    find_invalid_number(&parse_input(input), preamble_size)
}

pub fn day09b(input: &str, preamble_size: usize) -> i64 {
    let parsed_input = parse_input(input);
    let invalid_number = find_invalid_number(&parsed_input, preamble_size);
    (2..parsed_input.len())
        .find_map(|window_size| {
            parsed_input.windows(window_size).find_map(|group| {
                if group.iter().sum::<i64>() == invalid_number {
                    Some(group.iter().min().unwrap() + group.iter().max().unwrap())
                } else {
                    None
                }
            })
        })
        .unwrap()
}
