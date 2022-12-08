#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Instruction {
    fn from_char(ch: char) -> Self {
        match ch {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            'L' => Self::Left,
            'R' => Self::Right,
            'F' => Self::Forward,
            _ => panic!("Unexpected instruction: {}", ch),
        }
    }

    fn is_orientation(&self) -> bool {
        match self {
            Instruction::East | Instruction::West | Instruction::South | Instruction::North => true,
            _ => false,
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            Instruction::East => Orientation::East,
            Instruction::West => Orientation::West,
            Instruction::South => Orientation::South,
            Instruction::North => Orientation::North,
            _ => panic!("Cannot convert to orientation: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn position_change(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    fn shift_right(self) -> Orientation {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn orientation_change(self, instr: Instruction, value: i32) -> Self {
        match value {
            0 => self,
            180 => self.shift_right().shift_right(),
            90 => match instr {
                Instruction::Right => self.shift_right(),
                Instruction::Left => self.shift_right().shift_right().shift_right(),
                _ => panic!("Unexpected instruction for orientation change: {:?}", instr),
            },
            270 => match instr {
                Instruction::Right => self.shift_right().shift_right().shift_right(),
                Instruction::Left => self.shift_right(),
                _ => panic!("Unexpected instruction for orientation change: {:?}", instr),
            },
            _ => panic!("Unexpected orientation change value: {}", value),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<(Instruction, i32)> {
    input
        .trim_end()
        .split("\n")
        .map(|line| {
            (
                Instruction::from_char(line.chars().nth(0).unwrap()),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn manhattan_distance(x_pos: i32, y_pos: i32) -> i32 {
    x_pos.abs() + y_pos.abs()
}

fn rotate_waypoint_right(wp: (i32, i32)) -> (i32, i32) {
    (wp.1, -wp.0)
}

fn rotate_waypoint(wp: (i32, i32), dir: Instruction, value: i32) -> (i32, i32) {
    match value {
        0 => wp,
        180 => rotate_waypoint_right(rotate_waypoint_right(wp)),
        90 => match dir {
            Instruction::Right => rotate_waypoint_right(wp),
            Instruction::Left => {
                rotate_waypoint_right(rotate_waypoint_right(rotate_waypoint_right(wp)))
            }
            _ => panic!("Unexpected instruction for rotation: {:?}", dir),
        },
        270 => match dir {
            Instruction::Right => {
                rotate_waypoint_right(rotate_waypoint_right(rotate_waypoint_right(wp)))
            }
            Instruction::Left => rotate_waypoint_right(wp),
            _ => panic!("Unexpected instruction for rotation: {:?}", dir),
        },
        _ => panic!("Unexpected rotation value: {}", value),
    }
}

pub fn day12a(input: &str) -> i64 {
    let instructions = parse_instructions(input);
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut orientation = Orientation::East;

    for (instr, value) in instructions {
        if instr.is_orientation() {
            let (x_ch, y_ch) = instr.orientation().position_change();
            x_pos += x_ch * value;
            y_pos += y_ch * value;
        } else if instr == Instruction::Forward {
            let (x_ch, y_ch) = orientation.position_change();
            x_pos += x_ch * value;
            y_pos += y_ch * value;
        } else {
            orientation = orientation.orientation_change(instr, value);
        }
    }
    manhattan_distance(x_pos, y_pos) as i64
}

pub fn day12b(input: &str) -> i64 {
    let instructions = parse_instructions(input);
    let mut wp_x = 10;
    let mut wp_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for (instr, value) in instructions {
        if instr.is_orientation() {
            let (x_ch, y_ch) = instr.orientation().position_change();
            wp_x += x_ch * value;
            wp_y += y_ch * value;
        } else if instr == Instruction::Forward {
            ship_x += wp_x * value;
            ship_y += wp_y * value;
        } else {
            (wp_x, wp_y) = rotate_waypoint((wp_x, wp_y), instr, value);
        }
    }
    manhattan_distance(ship_x, ship_y) as i64
}
