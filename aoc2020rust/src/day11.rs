static ADJACENT_CELLS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Seat {
    fn symbol(&self) -> char {
        match self {
            Self::Empty => 'L',
            Self::Floor => '.',
            Self::Occupied => '#',
        }
    }

    fn from_symbol(symbol: char) -> Self {
        match symbol {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => panic!("Unexpected seat symbol: {}", symbol),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Seat>> {
    input
        .trim_end()
        .split("\n")
        .map(|line| Vec::from_iter(line.chars().map(|ch| Seat::from_symbol(ch))))
        .collect::<Vec<_>>()
}

fn count_occupied(map: &Vec<Vec<Seat>>, row: i32, col: i32) -> usize {
    ADJACENT_CELLS
        .iter()
        .map(|(row_o, col_o)| (row + row_o, col + col_o))
        .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < map.len() as i32 && *c < map[0].len() as i32)
        .map(|(r, c)| map[r as usize][c as usize].clone())
        .filter(|s| *s == Seat::Occupied)
        .count()
}

fn count_occupied_b(map: &Vec<Vec<Seat>>, row: i32, col: i32) -> usize {
    ADJACENT_CELLS
        .iter()
        .filter(|(row_mod, col_mod)| {
            let mut seats: Vec<Seat> = Vec::new();
            let mut curr_r = row;
            let mut curr_c = col;
            loop {
                curr_r += row_mod;
                curr_c += col_mod;
                if curr_r < 0
                    || curr_r >= map.len() as i32
                    || curr_c < 0
                    || curr_c >= map[0].len() as i32
                {
                    break;
                }
                seats.push(map[curr_r as usize][curr_c as usize].clone());
                if map[curr_r as usize][curr_c as usize] != Seat::Floor {
                    break;
                }
            }
            seats.iter().filter(|s| **s == Seat::Occupied).count() > 0
        })
        .count()
}

fn print_map(map: &Vec<Vec<Seat>>) {
    println!(
        "{}",
        map.iter()
            .map(|row| row
                .iter()
                .map(|s| s.symbol())
                .fold(String::new(), |accum, ch| format!("{}{}", accum, ch)))
            .reduce(|accum, line| accum + "\n" + &line)
            .unwrap()
    );
    println!();
}

pub fn day11a(input: &str, print_state: bool) -> i64 {
    let mut last_map = parse_input(input);
    let rows = last_map.len();
    let columns = last_map[0].len();
    let mut count = 0;
    let mut curr_map: Vec<Vec<Seat>>;
    let mut change: bool;
    loop {
        if print_state {
            print_map(&last_map);
        }
        change = false;
        curr_map = last_map.clone();
        for r in 0..rows {
            for c in 0..columns {
                let occupied = count_occupied(&last_map, r as i32, c as i32);
                curr_map[r][c] = match (last_map[r][c].clone(), occupied) {
                    (Seat::Floor, _) => Seat::Floor,
                    _ if occupied == 0 => Seat::Occupied,
                    _ if occupied >= 4 => Seat::Empty,
                    (seat, _) => seat,
                };
                if curr_map[r][c] != last_map[r][c] {
                    change = true;
                }
            }
        }
        if !change {
            break curr_map
                .iter()
                .map(|row| row.iter().filter(|s| **s == Seat::Occupied).count())
                .sum::<usize>() as i64;
        }
        count += 1;
        if count > 1000 {
            panic!("Simulation takes too long");
        }
        last_map = curr_map;
    }
}

pub fn day11b(input: &str, print_state: bool) -> i64 {
    let mut last_map = parse_input(input);
    let rows = last_map.len();
    let columns = last_map[0].len();
    let mut count = 0;
    let mut curr_map: Vec<Vec<Seat>>;
    let mut change: bool;
    loop {
        if print_state {
            print_map(&last_map);
        }
        change = false;
        curr_map = last_map.clone();
        for r in 0..rows {
            for c in 0..columns {
                let occupied = count_occupied_b(&last_map, r as i32, c as i32);
                curr_map[r][c] = match (last_map[r][c].clone(), occupied) {
                    (Seat::Floor, _) => Seat::Floor,
                    _ if occupied == 0 => Seat::Occupied,
                    _ if occupied >= 5 => Seat::Empty,
                    (seat, _) => seat,
                };
                if curr_map[r][c] != last_map[r][c] {
                    change = true;
                }
            }
        }
        if !change {
            break curr_map
                .iter()
                .map(|row| row.iter().filter(|s| **s == Seat::Occupied).count())
                .sum::<usize>() as i64;
        }
        count += 1;
        if count > 1000 {
            panic!("Simulation takes too long");
        }
        last_map = curr_map;
    }
}
