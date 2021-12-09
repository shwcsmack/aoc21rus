use std::str::FromStr;

const FILE: &str = "data/day2data.txt";

pub fn part_one() -> i32 {
    let commands = get_data();
    let mut position = Position::new();
    for command in &commands {
        position.run_part_one_command(command)
    }
    // println!("{:?}", &commands);
    // println!("horizontal {:?} depth {:?} answer {:?}", horizontal, depth, horizontal*depth);
    position.answer()
}

pub fn part_two() -> i32 {
    let commands = get_data();
    let mut position = Position::new();
    for command in &commands {
        position.run_part_two_command(command)
    }
    // println!("{:?}", &commands);
    // println!("horizontal {:?} depth {:?} answer {:?}", horizontal, depth, horizontal*depth);
    position.answer()
}

fn get_data() -> Vec<Command> {
    let commands: Vec<Command> = crate::util::read_file(FILE)
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect();
    commands
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn run_part_one_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => { self.horizontal += command.value }
            Direction::Down => { self.depth += command.value }
            Direction::Up => { self.depth -= command.value }
        }
    }

    fn run_part_two_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.horizontal += command.value;
                self.depth += self.aim * command.value;
            }
            Direction::Down => { self.aim += command.value }
            Direction::Up => { self.aim -= command.value }
        }
    }

    fn answer(&self) -> i32 {
        self.horizontal * self.depth
    }
}

#[derive(Debug, PartialEq)]
struct Command {
    direction: Direction,
    value: i32,
}

impl FromStr for Command {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let direction = parts[0].parse::<Direction>().expect("Couldnt parse Direction");
        let value = parts[1].parse::<i32>().expect("Couldnt parse value");
        Ok(Self { direction, value })
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();
        let lowercase = lowercase.as_str();

        match lowercase {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(String::from("No matching direction found")),
        }
    }
}