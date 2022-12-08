pub fn day02a(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n")
        .filter(|line| {
            let line_split: Vec<&str> = line.split_whitespace().collect();
            let (allowed_range, letter, password) =
                (line_split[0], &line_split[1][0..1], line_split[2]);
            let allowed_range_split: Vec<&str> = allowed_range.split("-").collect();
            let (min_allowed, max_allowed) = (
                allowed_range_split[0].parse::<i32>().unwrap(),
                allowed_range_split[1].parse::<i32>().unwrap(),
            );
            let letter_count = password.matches(letter).count() as i32;
            letter_count >= min_allowed && letter_count <= max_allowed
        })
        .count() as i64
}

pub fn day02b(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n")
        .filter(|line| {
            let line_split: Vec<&str> = line.split_whitespace().collect();
            let (indexes, letter, password) = (
                line_split[0],
                &line_split[1].chars().nth(0).unwrap(),
                line_split[2],
            );
            let allowed_range_split: Vec<&str> = indexes.split("-").collect();
            let (idx1, idx2) = (
                allowed_range_split[0].parse::<usize>().unwrap(),
                allowed_range_split[1].parse::<usize>().unwrap(),
            );
            let idx1_match = password.chars().nth(idx1 - 1).unwrap() == *letter;
            let idx2_match = password.chars().nth(idx2 - 1).unwrap() == *letter;
            (idx1_match && !idx2_match) || (!idx1_match && idx2_match)
        })
        .count() as i64
}
