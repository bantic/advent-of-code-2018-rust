#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point2 {
  pub x: u32,
  pub y: u32,
}
impl Point2 {
  pub fn new(x: u32, y: u32) -> Point2 {
    Point2 { x, y }
  }
  pub fn manhattan_dist(&self, o: &Self) -> u32 {
    ((self.x as i32 - o.x as i32).abs() + (self.y as i32 - o.y as i32).abs()) as u32
  }
}

pub fn scan_ints(s: &str) -> Vec<i32> {
  let mut result = vec![];
  let mut digit_chars = String::new();
  for c in s.chars() {
    match (c.is_numeric(), digit_chars.len()) {
      (true, _) => digit_chars.push(c),
      (false, len) if len > 0 => {
        result.push(digit_chars.parse().unwrap());
        digit_chars.clear();
      }
      _ => continue,
    }
  }
  if digit_chars.len() > 0 {
    result.push(digit_chars.parse().unwrap());
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scan() {
    assert_eq!(vec![1, 2, 3], scan_ints("adsf1,2    @3"))
  }
}
