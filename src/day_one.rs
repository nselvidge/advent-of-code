use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn measurement_larger_than_previous() -> String {
  let mut lines = get_day_one_data();

  let mut count = 0;
  let mut previous = lines.nth(0).unwrap();

  for current in lines {
    if current > previous {
      count = count + 1;
    }
    previous = current;
  }

  String::from(format!("{}", count))
}

pub fn sliding_window_measurement() -> String {
  let mut lines = get_day_one_data();

  let initial = lines.by_ref().take(3).collect();

  let mut queue = NumberQueue::new(initial);
  let mut previous_sum = queue.sum();
  let mut count = 0;

  for current in lines {
    queue.remove();
    queue.add(current);
    if previous_sum < queue.sum() {
      count = count + 1;
    }
    previous_sum = queue.sum()
  }

  String::from(format!("{}", count))
}

fn get_day_one_data() -> impl Iterator<Item = u32> {
  let path = Path::new("data/day_one_data.txt");
  let file = File::open(path).unwrap();
  io::BufReader::new(file)
    .lines()
    .map(|line| -> u32 { line.unwrap().parse().unwrap() })
}

struct NumberQueue {
  data: VecDeque<u32>,
}

impl NumberQueue {
  fn new(data: Vec<u32>) -> NumberQueue {
    NumberQueue {
      data: VecDeque::from(data),
    }
  }
  fn add(&mut self, item: u32) {
    self.data.push_back(item);
  }
  fn remove(&mut self) -> Option<u32> {
    self.data.pop_front()
  }
  fn sum(&self) -> u32 {
    let mut total = 0;
    for item in &self.data {
      total = total + item;
    }
    total
  }
}
