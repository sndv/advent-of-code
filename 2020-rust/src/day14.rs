use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Cmd<'a> {
    Mask(&'a str),
    Mem(i64, i64),
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .trim_end()
        .split("\n")
        .map(|line| {
            let (k, v) = line.split_once(" = ").unwrap();
            if k == "mask" {
                Cmd::Mask(v)
            } else {
                let addr = k
                    .trim_end_matches(']')
                    .split_once('[')
                    .unwrap()
                    .1
                    .parse::<i64>()
                    .unwrap();
                Cmd::Mem(addr, v.parse::<i64>().unwrap())
            }
        })
        .collect()
}

fn apply_mask_a(mask: &str, value: i64) -> i64 {
    let mask_0 = i64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
    let mask_1 = i64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
    (value | mask_0) & mask_1
}

fn mask_combinations(mask: &str) -> Vec<i64> {
    match mask.find('X') {
        Some(_) => {
            let mut r = mask_combinations(&mask.replacen('X', "0", 1));
            r.append(&mut mask_combinations(&mask.replacen('X', "1", 1)));
            r
        }
        None => Vec::from([i64::from_str_radix(mask, 2).unwrap()]),
    }
}

fn apply_mask_b(mask: &str, addr: i64) -> Vec<i64> {
    let mask_0 = i64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
    let new_addr = addr | mask_0;
    mask_combinations(&mask.replace('1', "0"))
        .iter()
        .map(|m| new_addr ^ m)
        .collect()
}

pub fn day14a(input: &str) -> i64 {
    let parsed_input = parse_input(input);
    let mut result = HashMap::new();
    let mut curr_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    for cmd in parsed_input {
        match cmd {
            Cmd::Mask(mask) => curr_mask = mask,
            Cmd::Mem(addr, value) => {
                result.insert(addr, apply_mask_a(curr_mask, value));
            }
        }
    }
    result.into_values().sum::<i64>()
}

pub fn day14b(input: &str) -> i64 {
    let parsed_input = parse_input(input);
    let mut result = HashMap::new();
    let mut curr_mask = "000000000000000000000000000000000000";
    for cmd in parsed_input {
        match cmd {
            Cmd::Mask(mask) => curr_mask = mask,
            Cmd::Mem(addr, value) => {
                for new_addr in apply_mask_b(curr_mask, addr) {
                    result.insert(new_addr, value);
                }
            }
        }
    }
    result.into_values().sum::<i64>()
}
