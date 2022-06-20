fn parse_input(input: &str) -> (i32, Vec<i32>) {
    let lines = input.trim_end().split("\n").collect::<Vec<&str>>();
    let arrival = lines[0].parse::<i32>().unwrap();
    let bus_ids = lines[1]
        .split(",")
        .filter(|el| *el != "x")
        .map(|bus_id| bus_id.parse::<i32>().unwrap())
        .collect();
    (arrival, bus_ids)
}

pub fn day13a(input: &str) -> i64 {
    let (arrival_time, bus_ids) = parse_input(input);
    let (bus_id, wait) = bus_ids
        .iter()
        .map(|bus_id| (bus_id, bus_id - (arrival_time % bus_id)))
        .reduce(|a, b| if a.1 < b.1 { a } else { b })
        .unwrap();
    (bus_id * wait).into()
}

fn reduce_2(id_1: i64, rem_1: i64, id_2: i64, rem_2: i64) -> (i64, i64) {
    let result: Vec<i64> = (0..10000000)
        .filter_map(|i| {
            let r = id_1 * i + rem_1;
            if r % id_2 == rem_2 {
                Some(r)
            } else {
                None
            }
        })
        .take(2)
        .collect();
    (result[1] - result[0], result[0])
}

pub fn day13b(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n")
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|el| el.1 != "x")
        .map(|(idx, bus_id_str)| {
            let bus_id = bus_id_str.parse::<i64>().unwrap();
            let idx_i64 = idx as i64;
            let expected_rem = (bus_id - (idx_i64 % bus_id)).abs() % bus_id;
            (bus_id, expected_rem)
        })
        .reduce(|a, b| reduce_2(a.0, a.1, b.0, b.1))
        .unwrap()
        .1
}
