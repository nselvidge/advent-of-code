mod day_one;

fn main() {
  println!(
    "Day 1, part 1 solution: {}",
    day_one::measurement_larger_than_previous()
  );
  println!(
    "Day 1, part 2 solution: {}",
    day_one::sliding_window_measurement()
  );
}
