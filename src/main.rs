mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

use utils::time;

fn main() {
  time("Advent of Code Problems", || {
    time("day1", day1::run);
    time("day2", day2::run);
    time("day3", day3::run);
    time("day4", day4::run);
    time("day5", day5::run);
    time("day6", day6::run);
    time("day7", day7::run);
  });
}
