use std::collections::HashSet;

pub fn day05a(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n")
        .map(|seat_code| {
            let seat_code_bin = seat_code
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            isize::from_str_radix(&seat_code_bin, 2).unwrap() as i64
        })
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap()
}

pub fn day05b(input: &str) -> i64 {
    let seat_ids: Vec<i64> = input
        .trim_end()
        .split("\n")
        .map(|seat_code| {
            let seat_code_bin = seat_code
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            isize::from_str_radix(&seat_code_bin, 2).unwrap() as i64
        })
        .collect();
    let first_seat = seat_ids
        .iter()
        .reduce(|accum, item| if accum <= item { accum } else { item })
        .unwrap();
    let last_seat = seat_ids
        .iter()
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap();
    let all_seats: HashSet<i64> = HashSet::from_iter(*first_seat..=*last_seat);
    let taken_seats = HashSet::from_iter(seat_ids);
    *(all_seats.difference(&taken_seats).nth(0).unwrap())
}
