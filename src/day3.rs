use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Claim {
  id: u32,
  x: u32,
  y: u32,
  w: u32,
  h: u32,
}
impl Claim {
  fn new(s: &str) -> Claim {
    let ints = utils::scan_ints(s);
    if ints.len() != 5 {
      panic!("Wrong ints");
    }
    Claim {
      id: ints[0] as u32,
      x: ints[1] as u32,
      y: ints[2] as u32,
      w: ints[3] as u32,
      h: ints[4] as u32,
    }
  }
  fn xmax(&self) -> u32 {
    self.x + self.w - 1
  }
  fn ymax(&self) -> u32 {
    self.y + self.h - 1
  }
}

pub fn run() {
  let contents = include_str!("../data/day3.txt");
  let claims = contents
    .lines()
    .map(|l| Claim::new(l))
    .collect::<Vec<Claim>>();
  println!("day 3 part 1: {}", part1(&claims));
}

#[derive(PartialEq)]
enum ClaimCount {
  Zero,
  One,
  Intersect,
}

fn part1(claims: &Vec<Claim>) -> usize {
  let mut grid = HashMap::<(u32, u32), ClaimCount>::new();

  for claim in claims {
    for x in (claim.x)..=(claim.xmax()) {
      for y in (claim.y)..=(claim.ymax()) {
        let entry = grid.entry((x, y)).or_insert(ClaimCount::Zero);
        match entry {
          ClaimCount::Zero => *entry = ClaimCount::One,
          ClaimCount::One => *entry = ClaimCount::Intersect,
          _ => {}
        }
      }
    }
  }

  grid
    .values()
    .filter(|v| **v == ClaimCount::Intersect)
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let contents = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
    let claims = contents.lines().map(|l| Claim::new(l)).collect();
    assert_eq!(part1(&claims), 4);
  }
}
