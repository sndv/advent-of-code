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

fn get_input(day: usize, input_n: usize) -> String {
    let file_suffix = match input_n {
        0 => String::from(""),
        in_n => format!("-ex{:02}", in_n),
    };
    let input_filename = format!("./input/day{:02}/input{}", day, file_suffix);
    let missing_msg = format!("Missing input file: {}", input_filename);
    fs::read_to_string(input_filename).expect(&missing_msg)
}

fn execute(day: usize, part: char, input_num: usize, expected: Option<i64>, run: fn(&str) -> i64) {
    let input = get_input(day, input_num);
    let result = run(&input);
    let result_suffix = match expected {
        Some(answer) => {
            if result == answer {
                String::from(" \u{2714}")
            } else {
                format!(" (expected: {})", answer)
            }
        }
        None => String::from(""),
    };
    let input_str = match input_num {
        0 => String::from("inpt"),
        in_n => format!("ex{:02}", in_n),
    };
    println!(
        "Day{:02}{} {}: {}{}",
        day, part, input_str, result, result_suffix
    );
}

fn full_run() {
    println!();
    execute(1, 'a', 1, Some(514579), day01::day01a);
    execute(1, 'a', 0, Some(842016), day01::day01a);
    execute(1, 'b', 1, Some(241861950), day01::day01b);
    execute(1, 'b', 0, Some(9199664), day01::day01b);

    println!();
    execute(2, 'a', 1, Some(2), day02::day02a);
    execute(2, 'a', 0, Some(655), day02::day02a);
    execute(2, 'b', 1, Some(1), day02::day02b);
    execute(2, 'b', 0, Some(673), day02::day02b);

    println!();
    execute(3, 'a', 1, Some(7), day03::day03a);
    execute(3, 'a', 0, Some(167), day03::day03a);
    execute(3, 'b', 1, Some(336), day03::day03b);
    execute(3, 'b', 0, Some(736527114), day03::day03b);

    println!();
    execute(4, 'a', 1, Some(2), day04::day04a);
    execute(4, 'a', 0, Some(228), day04::day04a);
    execute(4, 'b', 2, Some(0), day04::day04b);
    execute(4, 'b', 3, Some(4), day04::day04b);
    execute(4, 'b', 0, Some(175), day04::day04b);

    println!();
    execute(5, 'a', 1, Some(820), day05::day05a);
    execute(5, 'a', 0, Some(866), day05::day05a);
    execute(5, 'b', 0, Some(583), day05::day05b);

    println!();
    execute(6, 'a', 1, Some(11), day06::day06a);
    execute(6, 'a', 0, Some(6521), day06::day06a);
    execute(6, 'b', 1, Some(6), day06::day06b);
    execute(6, 'b', 0, Some(3305), day06::day06b);

    println!();
    execute(7, 'a', 1, Some(4), day07::day07a);
    execute(7, 'a', 0, Some(169), day07::day07a);
    execute(7, 'b', 1, Some(32), day07::day07b);
    execute(7, 'b', 2, Some(126), day07::day07b);
    execute(7, 'b', 0, Some(82372), day07::day07b);

    println!();
    execute(8, 'a', 1, Some(5), day08::day08a);
    execute(8, 'a', 0, Some(1451), day08::day08a);
    execute(8, 'b', 1, Some(8), day08::day08b);
    execute(8, 'b', 0, Some(1160), day08::day08b);
}

fn main() {
    full_run();
}