use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
  let start = Instant::now();
  day1::run();
  day2::run();
  day3::run();
  day4::run();
  day5::run();
  day6::run();
  day7::run();
  println!(
    "Finished running in {:?}",
    Instant::now().duration_since(start)
  );
}
