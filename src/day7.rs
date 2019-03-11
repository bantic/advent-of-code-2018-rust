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
  println!("day 7 part 2: {:?}", part2(&dep_map));
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

struct Worker {
  elapsed: u8,
  cur: Option<char>,
}

impl Worker {
  fn new() -> Worker {
    Worker {
      elapsed: 0,
      cur: None,
    }
  }

  fn work_time(c: char) -> u8 {
    60 + 1 + (c as u8) - b'A'
  }

  fn tick(&mut self) {
    self.elapsed += 1;
  }

  fn can_complete(&self) -> bool {
    if let Some(cur) = self.cur {
      let needed = Worker::work_time(cur);
      self.elapsed >= needed
    } else {
      false
    }
  }

  fn complete(&mut self) -> char {
    if !self.can_complete() {
      panic!("cannot complete");
    }
    let ch = self.cur.unwrap();
    self.cur = None;
    self.elapsed = 0;
    ch
  }

  fn can_work(&self) -> bool {
    self.cur == None
  }

  fn is_working(&self) -> bool {
    self.cur != None
  }

  fn work(&mut self, c: char) {
    self.elapsed = 0;
    self.cur = Some(c);
  }
}

struct Pool {
  elapsed: u32,
  workers: Vec<Worker>,
}

impl Pool {
  fn new(count: usize) -> Pool {
    let mut workers = vec![];
    for _ in 0..count {
      workers.push(Worker::new());
    }
    Pool {
      elapsed: 0,
      workers,
    }
  }

  fn available_workers(&self) -> usize {
    self.workers.iter().filter(|w| w.can_work()).count()
  }

  fn can_work(&self) -> bool {
    self.available_workers() > 0
  }

  fn work(&mut self, value: char) {
    let mut did_work = false;
    for worker in &mut self.workers {
      if worker.can_work() {
        worker.work(value);
        did_work = true;
        break;
      }
    }
    if !did_work {
      panic!("could not work");
    }
  }

  fn tick(&mut self) -> Vec<char> {
    self.elapsed += 1;
    let mut completed = vec![];
    for worker in &mut self.workers {
      if !worker.is_working() {
        continue;
      }
      worker.tick();
      if worker.can_complete() {
        completed.push(worker.complete());
      }
    }
    completed
  }
}

fn part2(dep_map: &DependencyMap) -> u32 {
  let mut seen: HashSet<char> = HashSet::new();
  let mut completed: HashSet<char> = HashSet::new();
  let mut next: Vec<char> = vec![];

  let mut pool = Pool::new(5);
  while completed.len() < dep_map.len() {
    next.sort();
    while pool.can_work() && !next.is_empty() {
      let v = next.remove(0);
      pool.work(v);
      seen.insert(v);
    }

    for ch in pool.tick() {
      completed.insert(ch);
    }

    for (&step, deps) in dep_map {
      if seen.contains(&step) || next.contains(&step) || completed.contains(&step) {
        continue;
      }
      let deps_len = deps.iter().filter(|d| !completed.contains(d)).count();
      if deps_len == 0 {
        next.push(step);
      }
    }
  }
  pool.elapsed - 1
}
