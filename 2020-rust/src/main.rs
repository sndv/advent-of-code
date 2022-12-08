use std::fs;
use std::option::Option;
use std::time;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn get_input(day: usize, input_n: usize) -> String {
    let file_suffix = match input_n {
        0 => String::from(""),
        in_n => format!("-ex{:02}", in_n),
    };
    let input_filename = format!("./input/day{:02}/input{}", day, file_suffix);
    let missing_msg = format!("Missing input file: {}", input_filename);
    fs::read_to_string(input_filename).expect(&missing_msg)
}

fn format_duration(nanos: u128) -> String {
    match nanos {
        en if en < 1_000 => format!("{}ns", en),
        en if en < 1_000_000 => format!("{:.2}μs", en as f64 / 1_000.0),
        en if en < 1_000_000_000 => format!("{:.2}ms", en as f64 / 1_000_000.0),
        en => format!("{:.2}s", en as f64 / 1_000_000_000.0),
    }
}
fn execute(day: usize, part: char, input_num: usize, expected: Option<i64>, run: fn(&str) -> i64) {
    let input = get_input(day, input_num);
    let start = time::Instant::now();
    let result = run(&input);
    let duration = start.elapsed().as_nanos();
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
        "Day{:02}{} {}: {} ({}){}",
        day,
        part,
        input_str,
        result,
        format_duration(duration),
        result_suffix
    );
}

fn run_all_days() {
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

    println!();
    execute(9, 'a', 1, Some(127), |input| day09::day09a(input, 5));
    execute(9, 'a', 0, Some(85848519), |input| day09::day09a(input, 25));
    execute(9, 'b', 1, Some(62), |input| day09::day09b(input, 5));
    execute(9, 'b', 0, Some(13414198), |input| day09::day09b(input, 25));

    println!();
    execute(10, 'a', 1, Some(35), day10::day10a);
    execute(10, 'a', 2, Some(220), day10::day10a);
    execute(10, 'a', 0, Some(2112), day10::day10a);
    execute(10, 'b', 1, Some(8), day10::day10b);
    execute(10, 'b', 2, Some(19208), day10::day10b);
    execute(10, 'b', 0, Some(3022415986688), day10::day10b);

    println!();
    execute(11, 'a', 1, Some(37), |input| day11::day11a(input, false));
    execute(11, 'a', 0, Some(2334), |input| day11::day11a(input, false));
    execute(11, 'b', 1, Some(26), |input| day11::day11b(input, false));
    execute(11, 'b', 0, Some(2100), |input| day11::day11b(input, false));

    println!();
    execute(12, 'a', 1, Some(25), day12::day12a);
    execute(12, 'a', 0, Some(757), day12::day12a);
    execute(12, 'b', 1, Some(286), day12::day12b);
    execute(12, 'b', 0, Some(51249), day12::day12b);

    println!();
    execute(13, 'a', 1, Some(295), day13::day13a);
    execute(13, 'a', 0, Some(2165), day13::day13a);
    execute(13, 'b', 1, Some(1068781), day13::day13b);
    execute(13, 'b', 2, Some(3417), day13::day13b);
    execute(13, 'b', 3, Some(754018), day13::day13b);
    execute(13, 'b', 4, Some(779210), day13::day13b);
    execute(13, 'b', 5, Some(1261476), day13::day13b);
    execute(13, 'b', 6, Some(1202161486), day13::day13b);
    execute(13, 'b', 0, Some(534035653563227), day13::day13b);

    println!();
    execute(14, 'a', 1, Some(165), day14::day14a);
    execute(14, 'a', 0, Some(4886706177792), day14::day14a);
    execute(14, 'b', 2, Some(208), day14::day14b);
    execute(14, 'b', 0, Some(3348493585827), day14::day14b);

    println!();
    execute(15, 'a', 1, Some(436), day15::day15a);
    execute(15, 'a', 2, Some(1), day15::day15a);
    execute(15, 'a', 3, Some(10), day15::day15a);
    execute(15, 'a', 4, Some(27), day15::day15a);
    execute(15, 'a', 5, Some(78), day15::day15a);
    execute(15, 'a', 6, Some(438), day15::day15a);
    execute(15, 'a', 7, Some(1836), day15::day15a);
    execute(15, 'a', 0, Some(211), day15::day15a);
    execute(15, 'b', 1, Some(175594), day15::day15b);
    execute(15, 'b', 2, Some(2578), day15::day15b);
    execute(15, 'b', 3, Some(3544142), day15::day15b);
    execute(15, 'b', 4, Some(261214), day15::day15b);
    execute(15, 'b', 5, Some(6895259), day15::day15b);
    execute(15, 'b', 6, Some(18), day15::day15b);
    execute(15, 'b', 7, Some(362), day15::day15b);
    execute(15, 'b', 0, Some(2159626), day15::day15b);

    println!();
    execute(16, 'a', 1, Some(71), day16::day16a);
    execute(16, 'a', 0, Some(20091), day16::day16a);
    execute(16, 'b', 2, Some(1716), |input| day16::day16b(input, ""));
    execute(16, 'b', 0, Some(2325343130651), |input| {
        day16::day16b(input, "departure")
    });
}

fn full_run() {
    let start = time::Instant::now();
    run_all_days();
    let duration = start.elapsed().as_nanos();
    println!();
    println!("Total time: {}", format_duration(duration));
}
fn main() {
    full_run();
}
