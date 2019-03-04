use std::collections::HashMap;

pub fn run() {
  let contents = include_str!("../data/day2.txt");
  part1(&contents);
  part2(&contents);
}

type SeenMap = HashMap<char, u8>;

fn part1(contents: &str) {
  let (two, three) = contents.lines().fold((0, 0), |(two, three), line| {
    let counts = char_counts(line);

    let values = counts.values().cloned().collect::<Vec<u8>>();
    match (values.contains(&2), values.contains(&3)) {
      (true, true) => (two + 1, three + 1),
      (false, true) => (two, three + 1),
      (true, false) => (two + 1, three),
      (false, false) => (two, three),
    }
  });
  println!("day 2 part 1: {}", two * three);
}

fn char_counts(s: &str) -> SeenMap {
  s.chars().fold(SeenMap::new(), |mut hash, ch| {
    *hash.entry(ch).or_default() += 1;
    hash
  })
}

fn common_chars(lhs: &str, rhs: &str) -> String {
  let mut result = String::with_capacity(lhs.len() - 1);
  for (lchar, rchar) in lhs.chars().zip(rhs.chars()) {
    if lchar == rchar {
      result.push(lchar);
    }
  }
  result
}

fn part2(contents: &str) {
  for line in contents.lines() {
    for other in contents.lines() {
      let mut diff_count = 0;
      for (l, r) in line.chars().zip(other.chars()) {
        if l != r {
          diff_count += 1;
          if diff_count > 1 {
            break;
          }
        }
      }
      if diff_count == 1 {
        println!("day 2 part 2: {}", common_chars(&line, &other));
        return;
      }
    }
  }
  println!("found nothing");
}
