use std::fs;
use std::option::Option;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

static DAYS_INFO: &[(
    for<'r> fn(&'r str) -> i64,
    for<'r> fn(&'r str) -> i64,
    i32,
    Option<i64>,
    Option<i64>,
    &[Option<i64>; 4],
    &[Option<i64>; 4],
)] = &[
    (
        day01::day01a,                        // Solve function for A
        day01::day01b,                        // Solve function for B
        1,                                    // Number of example inputs
        Some(842016),                         // Correct answer for A
        Some(9199664),                        // Correct answer for B
        &[Some(514579), None, None, None],    // Correct answers for A for example inputs
        &[Some(241861950), None, None, None], // Correct answers for B for example inputs
    ),
    (
        day02::day02a,
        day02::day02b,
        1,
        Some(655),
        Some(673),
        &[Some(2), None, None, None],
        &[Some(1), None, None, None],
    ),
    (
        day03::day03a,
        day03::day03b,
        1,
        Some(167),
        Some(736527114),
        &[Some(7), None, None, None],
        &[Some(336), None, None, None],
    ),
    (
        day04::day04a,
        day04::day04b,
        3,
        Some(228),
        Some(175),
        &[Some(2), Some(4), Some(4), None],
        &[Some(2), Some(0), Some(4), None],
    ),
    (
        day05::day05a,
        day05::day05b,
        1,
        Some(866),
        Some(583),
        &[Some(820), None, None, None],
        &[None, None, None, None],
    ),
    (
        day06::day06a,
        day06::day06b,
        1,
        Some(6521),
        Some(3305),
        &[Some(11), None, None, None],
        &[Some(6), None, None, None],
    ),
    (
        day07::day07a,
        day07::day07b,
        2,
        Some(169),
        Some(82372),
        &[Some(4), None, None, None],
        &[Some(32), Some(126), None, None],
    ),
    (
        day08::day08a,
        day08::day08b,
        1,
        Some(1451),
        Some(1160),
        &[Some(5), None, None, None],
        &[Some(8), None, None, None],
    ),
];

fn get_input(day: i32, file_suffix: &str) -> String {
    let input_filename = format!("./input/day{:02}/input{}", day, file_suffix);
    let missing_msg = format!("Missing input file: {}", input_filename);
    fs::read_to_string(input_filename).expect(&missing_msg)
}

fn check_result(result: i64, expected: Option<i64>) -> String {
    match expected {
        Some(answer) => {
            if result == answer {
                String::from("\u{2714}")
            } else {
                format!("(expected: {})", answer)
            }
        }
        None => String::from(""),
    }
}

fn execute_for_day(
    day: i32,
    solve_fn_a: fn(&str) -> i64,
    solve_fn_b: fn(&str) -> i64,
    input: &str,
    example_inputs: Vec<String>,
    answer_a: Option<i64>,
    answer_b: Option<i64>,
    example_answers_a: &[Option<i64>; 4],
    example_answers_b: &[Option<i64>; 4],
) {
    for (i, ex_input) in example_inputs.iter().enumerate() {
        let result = solve_fn_a(&ex_input);
        println!(
            "Day{:02}a ex{:02}: {} {}",
            day,
            i + 1,
            result,
            check_result(result, example_answers_a[i])
        );
    }
    let result_a = solve_fn_a(&input);
    println!(
        "Day{:02}a inpt: {} {}",
        day,
        result_a,
        check_result(result_a, answer_a)
    );
    for (i, ex_input) in example_inputs.iter().enumerate() {
        let result = solve_fn_b(&ex_input);
        println!(
            "Day{:02}b ex{:02}: {} {}",
            day,
            i + 1,
            result,
            check_result(result, example_answers_b[i])
        );
    }
    let result_b = solve_fn_b(&input);
    println!(
        "Day{:02}b inpt: {} {}",
        day,
        result_b,
        check_result(result_b, answer_b)
    );
    println!();
}

fn run_day(day: i32) {
    let day_idx = (day - 1) as usize;
    let day_info = DAYS_INFO[day_idx];
    let input = get_input(day, "");
    let ex_inputs = (1..(day_info.2 + 1))
        .map(|input_n| get_input(day, &(format!("-ex{:02}", input_n))))
        .collect();
    execute_for_day(
        day, day_info.0, day_info.1, &input, ex_inputs, day_info.3, day_info.4, day_info.5,
        day_info.6,
    );
}

fn full_run(last_day: i32) {
    for day in 1..last_day + 1 {
        run_day(day);
    }
}

fn main() {
    println!();
    full_run(8);
    // run_day(8);
    // println!("{}", day08::day08a(&get_input(8, "-ex01")));
}
