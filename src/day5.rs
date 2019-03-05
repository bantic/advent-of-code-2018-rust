fn prepare_input<'a>(c: &'a str) -> &'a str {
  c.trim()
}

pub fn run() {
  let contents = include_str!("../data/day5.txt");
  let contents = prepare_input(contents);
  println!("day 5 part 1: {}", part1(contents));
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
