use std::collections::HashSet;

pub fn day06a(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n\n")
        .map(|gr_answ| {
            let answ_set: HashSet<char> =
                HashSet::from_iter(gr_answ.replace(" ", "").replace("\n", "").chars());
            answ_set.len()
        })
        .reduce(|accum, item| accum + item)
        .unwrap() as i64
}

pub fn day06b(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n\n")
        .map(|gr_answ| {
            gr_answ
                .split("\n")
                .map(|line| HashSet::from_iter(line.chars()))
                .reduce(|accum: HashSet<char>, item| {
                    accum.intersection(&item).map(|ch| *ch).collect()
                })
                .unwrap()
                .len()
        })
        .reduce(|accum, item| accum + item)
        .unwrap() as i64
}
