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
  println!("day 6 part 2: {}", part2(&points));
}

fn get_bounds(points: &[Point2]) -> (u32, u32, u32, u32) {
  let xmin = points.iter().min_by_key(|p| p.x).unwrap().x;
  let xmax = points.iter().max_by_key(|p| p.x).unwrap().x;
  let ymin = points.iter().min_by_key(|p| p.y).unwrap().y;
  let ymax = points.iter().max_by_key(|p| p.y).unwrap().y;
  (xmin, xmax, ymin, ymax)
}

fn part1(points: &[Point2]) -> u32 {
  let (xmin, xmax, ymin, ymax) = get_bounds(points);

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

fn total_distance(x: u32, y: u32, points: &[Point2]) -> u32 {
  let mut total = 0;
  let cur = Point2::new(x, y);
  for point in points {
    total += cur.manhattan_dist(point);
  }
  total
}

fn part2(points: &[Point2]) -> u32 {
  const MIN_REGION_DIST: u32 = 10000;
  let (xmin, xmax, ymin, ymax) = get_bounds(points);
  let mut sum = 0;
  for x in xmin..=xmax {
    for y in ymin..=ymax {
      let dist = total_distance(x, y, points);
      if dist < MIN_REGION_DIST {
        sum += 1;
      }
    }
  }
  sum
}
