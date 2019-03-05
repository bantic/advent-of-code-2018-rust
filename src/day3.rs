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
    self.x + self.w
  }
  fn ymax(&self) -> u32 {
    self.y + self.h
  }
  fn overlaps(&self, other: &Claim) -> bool {
    let xmin = self.x.max(other.x);
    let ymin = self.y.max(other.y);
    let xmax = self.xmax().min(other.xmax());
    let ymax = self.ymax().min(other.ymax());

    (xmin <= self.xmax() && xmin <= other.xmax())
      && (ymin <= self.ymax() && ymin <= other.ymax())
      && (xmax >= self.x && xmax >= other.x)
      && (ymax >= self.y && ymax >= other.y)
      && (xmin < xmax)
      && (ymin < ymax)
  }
}

pub fn run() {
  let contents = include_str!("../data/day3.txt");
  let claims = contents
    .lines()
    .map(|l| Claim::new(l))
    .collect::<Vec<Claim>>();
  println!("day 3 part 1: {}", part1(&claims));
  println!("day 3 part 2: {}", part2(&claims).unwrap());
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
    for x in (claim.x)..(claim.xmax()) {
      for y in (claim.y)..(claim.ymax()) {
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

fn part2(claims: &Vec<Claim>) -> Option<u32> {
  for claim in claims {
    let mut intersects = false;
    for other in claims {
      if claim.id == other.id {
        continue;
      }
      if claim.overlaps(other) {
        intersects = true;
        break;
      }
    }
    if !intersects {
      return Some(claim.id);
    }
  }
  None
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

  #[test]
  fn overlaps() {
    let contents = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
    let claims: Vec<Claim> = contents.lines().map(|l| Claim::new(l)).collect();
    assert_eq!(claims[0].overlaps(&claims[1]), true);
    assert_eq!(claims[1].overlaps(&claims[2]), false);
  }

  #[test]
  fn overlaps2() {
    let c1 = Claim {
      id: 0,
      x: 0,
      y: 0,
      w: 2,
      h: 2,
    };
    let mut c2 = Claim {
      id: 0,
      x: 2,
      y: 2,
      w: 2,
      h: 2,
    };
    assert_eq!(c1.overlaps(&c2), false);
    c2.x = 1;
    assert_eq!(c1.overlaps(&c2), false);
    c2.y = 1;
    assert_eq!(c1.overlaps(&c2), true);
  }
}
