use std::collections::HashSet;

pub fn run() {
  let contents = include_str!("../data/day1.txt");
  let values = contents.lines().map(|l| l.parse::<i32>().unwrap());
  println!("day 1 part 1: {}", values.clone().sum::<i32>());

  let mut seen = HashSet::<i32>::default();
  let mut freq = 0;
  for v in values.clone().cycle() {
    freq += v;
    let inserted = seen.insert(freq);
    if !inserted {
      break;
    }
  }
  println!("day 1 part 2: {}", freq);
}
