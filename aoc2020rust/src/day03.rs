pub fn day03a(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n")
        .enumerate()
        .filter(|(i, line)| line.chars().nth(i * 3 % line.len()).unwrap() == '#')
        .count() as i64
}

fn hit_trees(input: &str, right: usize, down: usize) -> i64 {
    input
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(i, line)| if i % down == 0 { Some(line) } else { None })
        .enumerate()
        .filter(|(i, line)| line.chars().nth(i * right % line.len()).unwrap() == '#')
        .count() as i64
}

pub fn day03b(input: &str) -> i64 {
    let r_1_1 = hit_trees(input, 1, 1);
    let r_3_1 = hit_trees(input, 3, 1);
    let r_5_1 = hit_trees(input, 5, 1);
    let r_7_1 = hit_trees(input, 7, 1);
    let r_1_2 = hit_trees(input, 1, 2);
    r_1_1 * r_3_1 * r_5_1 * r_7_1 * r_1_2
}
