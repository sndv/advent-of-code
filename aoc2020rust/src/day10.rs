static COMBINATIONS_MAP: [i64; 8] = [1, 1, 2, 4, 7, 13, 24, 43];

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim_end()
        .split("\n")
        .map(|el| el.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn day10a(input: &str) -> i64 {
    let mut parsed_input = parse_input(input);
    parsed_input.push(0);
    parsed_input.sort();
    let differences =
        parsed_input
            .windows(2)
            .map(|sl| sl[1] - sl[0])
            .fold((0, 1), |(c_1, c_3), el| match el {
                1 => (c_1 + 1, c_3),
                3 => (c_1, c_3 + 1),
                _ => (c_1, c_3),
            });
    differences.0 * differences.1
}

pub fn day10b(input: &str) -> i64 {
    let mut parsed_input = parse_input(input);
    parsed_input.push(0);
    parsed_input.sort();
    parsed_input
        .windows(2)
        .map(|sl| sl[1] - sl[0])
        .fold(Vec::from([0]), |mut c, el| {
            if el == 1 {
                let new_value = c.pop().unwrap() + 1;
                c.push(new_value);
            } else if *(c.last().unwrap()) != 0 {
                c.push(0);
            }
            c
        })
        .iter()
        .map(|el| COMBINATIONS_MAP[*el as usize])
        .product()
}
