fn prepare_input(c: &str) -> &str {
  c.trim()
}

pub fn run() {
  let contents = include_str!("../data/day5.txt");
  let contents = prepare_input(contents);
  println!("day 5 part 1: {}", part1(contents));
  println!("day 5 part 2: {}", part2(contents));
}

fn reactive(lhs: char, rhs: char) -> bool {
  lhs.to_ascii_lowercase() == rhs.to_ascii_lowercase() && (lhs as i8 - rhs as i8).abs() == 0x20
}

fn part1(contents: &str) -> usize {
  let mut stack: Vec<char> = vec![];
  for next in contents.chars() {
    match stack.last() {
      Some(&prev) if reactive(prev, next) => {
        stack.pop();
      }
      Some(_) => {
        stack.push(next);
      }
      None => {
        stack.push(next);
      }
    }
  }

  stack.len()
}

fn part2(contents: &str) -> usize {
  let mut min = contents.len();
  // TODO - Maybe faster by checking for presence of the char first?
  // It is an unneeded loop if the char in question isn't in the output
  // Also, the maximum amount of reactive shortenings is 1/2 the # of upper and lower
  // occurrences of the char, so could also maybe use a regex to count and skip strings
  // that don't contain enough to beat the current `min`
  for c in b'a'..=b'z' {
    let c = c as char;
    let test = contents.replace(c, "").replace(c.to_ascii_uppercase(), "");
    let len = part1(&test);
    if len < min {
      min = len;
    }
  }
  min
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_react() {
    assert!(reactive('a', 'A'));
    assert!(reactive('A', 'a'));
    assert_eq!(reactive('b', 'b'), false);
    assert_eq!(reactive('B', 'B'), false);
    assert_eq!(reactive('b', 'c'), false);
  }

  #[test]
  fn example1() {
    let contents = prepare_input("dabAcCaCBAcCcaDA");
    assert_eq!(part1(contents), 10);
  }
}
