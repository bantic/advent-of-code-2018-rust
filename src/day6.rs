use crate::utils::{scan_ints, Point2};
use std::collections::{HashMap, HashSet};

fn prepare_input(s: &str) -> Vec<Point2> {
  s.lines()
    .map(|l| {
      let ints = scan_ints(l);
      Point2::new(ints[0] as u32, ints[1] as u32)
    })
    .collect()
}

pub fn run() {
  let contents = include_str!("../data/day6.txt");
  let points = prepare_input(contents);
  println!("day 6 part 1: {}", part1(&points));
}

fn part1(points: &Vec<Point2>) -> u32 {
  let xmin = points.iter().min_by_key(|p| p.x).unwrap().x;
  let xmax = points.iter().max_by_key(|p| p.x).unwrap().x;
  let ymin = points.iter().min_by_key(|p| p.y).unwrap().y;
  let ymax = points.iter().max_by_key(|p| p.y).unwrap().y;

  // keep track of total owned squares for each point
  // keep track of which points' extents touch the grid edge, to disregard them
  let mut points_touch_edge = HashSet::<usize>::new();
  let mut owned_squares_per_point = HashMap::<usize, u32>::new();

  let is_boundary = |x: u32, y: u32| -> bool { x == xmin || x == xmax || y == ymin || y == ymax };

  for x in xmin..=xmax {
    for y in ymin..=ymax {
      let mut min_dist = u32::max_value();
      let mut closest_point_idx = 0;
      let mut is_tie = false;
      let cur_point = Point2::new(x, y);
      for (idx, point) in points.iter().enumerate() {
        let dist = cur_point.manhattan_dist(point);
        if dist < min_dist {
          is_tie = false;
          min_dist = dist;
          closest_point_idx = idx;
        } else if dist == min_dist {
          is_tie = true;
        }
      }
      if !is_tie {
        let entry = owned_squares_per_point
          .entry(closest_point_idx)
          .or_default();
        *entry += 1;
        if is_boundary(x, y) {
          points_touch_edge.insert(closest_point_idx);
        }
      }
    }
  }

  let mut max_area = 0;
  for (idx, area) in owned_squares_per_point {
    if points_touch_edge.contains(&idx) {
      continue;
    }
    if area > max_area {
      max_area = area;
    }
  }

  max_area
}
