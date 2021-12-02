use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn depth_by_distance() -> i32 {
  let commands = get_day_two_data();
  let mut forward_units: i32 = 0;
  let mut depth_units: i32 = 0;
  for command in commands {
    if command.direction == "up" {
      depth_units -= command.units;
    } else if command.direction == "down" {
      depth_units += command.units;
    } else {
      forward_units += command.units;
    }
  }
  forward_units * depth_units
}

pub fn depth_by_distance_with_aim() -> i32 {
  let commands = get_day_two_data();
  let mut forward_units: i32 = 0;
  let mut aim_units: i32 = 0;
  let mut depth_units: i32 = 0;
  for command in commands {
    if command.direction == "up" {
      aim_units -= command.units;
    } else if command.direction == "down" {
      aim_units += command.units;
    } else {
      forward_units += command.units;
      depth_units += aim_units * command.units;
    }
  }
  forward_units * depth_units
}

struct Command {
  direction: String,
  units: i32,
}

impl Command {
  pub fn new(line: String) -> Command {
    let split: Vec<&str> = line.split(" ").collect();
    let direction = String::from(split[0]);
    let units: i32 = split[1].parse().unwrap();
    Command { direction, units }
  }
}

fn get_day_two_data() -> impl Iterator<Item = Command> {
  let path = Path::new("data/day_two_data.txt");
  let file = File::open(path).expect("File not found");
  io::BufReader::new(file)
    .lines()
    .map(|line| Command::new(line.unwrap()))
}
