pub fn day01a(input: &str) -> i64 {
    let r: Vec<i64> = input
        .split_whitespace()
        .map(|el| el.parse::<i64>().unwrap())
        .collect();
    for (i, first) in r.iter().enumerate() {
        for (j, second) in r.iter().enumerate() {
            if i != j && first + second == 2020 {
                return first * second;
            }
        }
    }
    -1
}

pub fn day01b(input: &str) -> i64 {
    let r: Vec<i64> = input
        .split_whitespace()
        .map(|el| el.parse::<i64>().unwrap())
        .collect();
    for (i, first) in r.iter().enumerate() {
        for (j, second) in r.iter().enumerate() {
            for (k, third) in r.iter().enumerate() {
                if i != j && i != k && j != k && first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    -1
}
