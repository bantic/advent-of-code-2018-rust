use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Dependency {
  step: char,
  dep: char,
}

type DependencyMap = HashMap<char, HashSet<char>>;

pub fn run() {
  let contents = fs::read_to_string("./data/day7.txt").unwrap();
  let deps: Vec<Dependency> = contents.lines().map(|l| parse_line(l)).collect();
  let mut dep_map: DependencyMap = HashMap::new();
  for dep in deps {
    dep_map.entry(dep.step).or_default().insert(dep.dep);
    dep_map.entry(dep.dep).or_default();
  }
  println!("day 7 part 1: {}", part1(&dep_map));
}

fn parse_line(l: &str) -> Dependency {
  let re = Regex::new(r"Step (.) must .* before step (.)").unwrap();
  let caps = re.captures(l).unwrap();

  Dependency {
    dep: caps.get(1).unwrap().as_str().chars().next().unwrap(),
    step: caps.get(2).unwrap().as_str().chars().next().unwrap(),
  }
}

fn part1(dep_map: &DependencyMap) -> String {
  let mut seen: HashSet<char> = HashSet::new();
  let mut used: Vec<char> = Vec::with_capacity(dep_map.len());
  let mut next: Vec<char> = vec![];

  while used.len() < dep_map.len() {
    next.sort();
    if !next.is_empty() {
      let v = next.remove(0);
      used.push(v);
      seen.insert(v);
    }
    for (&step, deps) in dep_map {
      if seen.contains(&step) || next.contains(&step) {
        continue;
      }
      let deps_len = deps.iter().filter(|d| !seen.contains(d)).count();
      if deps_len == 0 {
        next.push(step);
      }
    }
  }

  used.iter().collect::<String>()
}
