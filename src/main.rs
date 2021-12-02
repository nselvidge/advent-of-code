mod day_one;
mod day_two;

fn main() {
  println!(
    "Day 1, part 1 solution: {}",
    day_one::measurement_larger_than_previous()
  );
  println!(
    "Day 1, part 2 solution: {}",
    day_one::sliding_window_measurement()
  );
  println!("Day 2, part 1 solution: {}", day_two::depth_by_distance());
  println!(
    "Day 2, part 2 solution: {}",
    day_two::depth_by_distance_with_aim()
  );
}
