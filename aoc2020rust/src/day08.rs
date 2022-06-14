use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Acc,
    Jump,
    Noop,
}

fn parse_statements(input: &str) -> Vec<(Command, i32)> {
    input
        .trim_end()
        .split("\n")
        .map(|line| {
            let (command_text, value_text) = line.split_once(" ").unwrap();
            let value = value_text.parse::<i32>().unwrap();
            let command = match command_text {
                "acc" => Command::Acc,
                "jmp" => Command::Jump,
                "nop" => Command::Noop,
                _ => panic!("Unknown command: {}", command_text),
            };
            (command, value)
        })
        .collect()
}

fn run_program(statements: Vec<(Command, i32)>, allow_out_of_range: bool) -> (bool, i64) {
    let mut executed: HashSet<usize> = HashSet::new();
    let mut accumulator: i64 = 0;
    let mut next_statement: i32 = 0;
    let success = loop {
        if executed.contains(&(next_statement as usize)) {
            break false;
        }
        if next_statement == statements.len() as i32 {
            break true;
        }
        if next_statement > statements.len() as i32 || next_statement < 0 {
            if allow_out_of_range {
                break false;
            } else {
                panic!(
                    "Unexpected jump to satement {} of {}",
                    next_statement,
                    statements.len()
                );
            }
        }
        executed.insert(next_statement as usize);
        let (cmd, value) = &statements[next_statement as usize];
        match cmd {
            Command::Acc => {
                accumulator += *value as i64;
                next_statement += 1
            }
            Command::Jump => next_statement = next_statement + *value,
            Command::Noop => next_statement += 1,
        };
    };
    (success, accumulator)
}

pub fn day08a(input: &str) -> i64 {
    let statements = parse_statements(input);
    let (success, result) = run_program(statements, false);
    assert_eq!(success, false);
    result
}

fn change_command(
    statements: &Vec<(Command, i32)>,
    idx: usize,
    new_cmd: Command,
) -> Vec<(Command, i32)> {
    statements
        .iter()
        .enumerate()
        .map(|(i, (cmd, value))| {
            if i == idx {
                (new_cmd.clone(), *value)
            } else {
                (cmd.clone(), *value)
            }
        })
        .collect()
}

pub fn day08b(input: &str) -> i64 {
    let statements = parse_statements(input);
    for i in 0..statements.len() {
        let new_cmd = match statements[i].0 {
            Command::Acc => continue,
            Command::Jump => Command::Noop,
            Command::Noop => Command::Jump,
        };
        let (success, result) = run_program(change_command(&statements, i, new_cmd), true);
        if success {
            return result;
        } else {
            continue;
        }
    }
    -999
}
