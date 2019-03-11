use crate::utils;
use std::cmp::Ordering;
use std::collections::HashMap;

fn prepare_input(contents: &str) -> HashMap<u32, HashMap<u8, u32>> {
  let mut entries: Vec<Entry> = contents.lines().map(|l| Entry::new(l)).collect();
  entries.sort();
  guard_to_minute_map(&entries)
}

pub fn run() {
  let contents = include_str!("../data/day4.txt");
  let guard_id_to_minutes = prepare_input(contents);
  println!("day 4 part 1: {}", part1(&guard_id_to_minutes));
  println!("day 4 part 2: {}", part2(&guard_id_to_minutes));
}

#[derive(PartialEq, Eq, Debug, PartialOrd)]
enum EntryKind {
  Guard(u32),
  Sleep,
  Wake,
}

#[derive(PartialEq, Eq, Debug, PartialOrd)]
struct Entry {
  time: Time,
  kind: EntryKind,
}

impl Entry {
  fn new(s: &str) -> Entry {
    let ints = utils::scan_ints(s);
    let year = ints[0] as u32;
    let month = ints[1] as u8;
    let day = ints[2] as u8;
    let hours = ints[3] as u8;
    let minutes = ints[4] as u8;
    let kind = match ints.len() {
      5 => match (s.contains("falls asleep"), s.contains("wakes up")) {
        (true, false) => EntryKind::Sleep,
        (false, true) => EntryKind::Wake,
        _ => panic!("unexpected value"),
      },
      6 => EntryKind::Guard(ints[5] as u32),
      _ => panic!("error parsing"),
    };
    let time = Time {
      year,
      month,
      day,
      hours,
      minutes,
    };

    Entry { time, kind }
  }
}

#[derive(PartialEq, Eq, Debug)]
struct TimeSpan {
  start: Time,
  end: Time,
}

impl TimeSpan {
  fn from(start: &Time, end: &Time) -> TimeSpan {
    if start.hours != end.hours || start.day != end.day {
      panic!("hour and day differ");
    }
    TimeSpan {
      start: start.clone(),
      end: end.clone(),
    }
  }
}

impl Iterator for TimeSpan {
  type Item = Time;
  fn next(&mut self) -> Option<Time> {
    if self.start.minutes == self.end.minutes {
      None
    } else {
      let time = self.start.clone();
      self.start.minutes += 1;
      Some(time)
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Time {
  year: u32,
  month: u8,
  day: u8,
  hours: u8,
  minutes: u8,
}

impl Time {
  fn until(&self, o: &Self) -> Option<TimeSpan> {
    match self.cmp(&o) {
      Ordering::Greater => None,
      _ => Some(TimeSpan::from(self, o)),
    }
  }
}

impl Ord for Entry {
  fn cmp(&self, o: &Self) -> Ordering {
    self.time.cmp(&o.time)
  }
}

impl Ord for Time {
  fn cmp(&self, o: &Self) -> Ordering {
    self
      .year
      .cmp(&o.year)
      .then(self.month.cmp(&o.month))
      .then(self.day.cmp(&o.day))
      .then(self.hours.cmp(&o.hours))
      .then(self.minutes.cmp(&o.minutes))
  }
}
impl PartialOrd for Time {
  fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
    Some(self.cmp(o))
  }
}

fn guard_to_minute_map(entries: &[Entry]) -> HashMap<u32, HashMap<u8, u32>> {
  let mut guard_id_to_minutes: HashMap<u32, HashMap<u8, u32>> = HashMap::new();
  let mut cur_guard_id: u32 = match entries.first() {
    Some(Entry {
      kind: EntryKind::Guard(guard_id),
      ..
    }) => *guard_id,
    _ => panic!("First entry should be a guard"),
  };

  let mut cur_time = match entries.first() {
    Some(Entry { time, .. }) => time,
    _ => panic!("No minutes in first entry"),
  };

  for entry in entries.iter().skip(1) {
    match entry.kind {
      EntryKind::Guard(id) => cur_guard_id = id,
      EntryKind::Sleep => {
        cur_time = &entry.time;
      }
      EntryKind::Wake => {
        let minutes_map = guard_id_to_minutes
          .entry(cur_guard_id)
          .or_insert_with(HashMap::new);
        for time in cur_time.until(&entry.time).unwrap() {
          let minutes_entry = minutes_map.entry(time.minutes).or_default();
          *minutes_entry += 1;
        }
      }
    }
  }
  guard_id_to_minutes
}

fn part1(guard_id_to_minutes: &HashMap<u32, HashMap<u8, u32>>) -> u32 {
  let mut max_total = 0;
  let mut max_guard_id = 0;
  for (guard_id, minutes_map) in guard_id_to_minutes {
    let total: u32 = minutes_map.values().sum();
    if total > max_total {
      max_total = total;
      max_guard_id = *guard_id;
    }
  }

  let max_minute_map = guard_id_to_minutes.get(&max_guard_id).unwrap();
  let mut max_count = 0;
  let mut max_minute = 0;
  for (&minute, &count) in max_minute_map {
    if count > max_count {
      max_count = count;
      max_minute = minute;
    }
  }

  max_guard_id * u32::from(max_minute)
}

fn part2(guard_id_to_minutes: &HashMap<u32, HashMap<u8, u32>>) -> u32 {
  let mut max_count = 0;
  let mut max_minute = 0;
  let mut max_guard_id = 0;
  for (&guard_id, minutes_map) in guard_id_to_minutes {
    for (&minute, &count) in minutes_map {
      if count > max_count {
        max_count = count;
        max_minute = minute;
        max_guard_id = guard_id;
      }
    }
  }

  max_guard_id * u32::from(max_minute)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let contents = include_str!("../data/day4-example.txt");
    let map = prepare_input(contents);
    assert_eq!(part1(&map), 240);
  }

  #[test]
  fn example2() {
    let contents = include_str!("../data/day4-example.txt");
    let map = prepare_input(contents);
    assert_eq!(part2(&map), 4455);
  }
}
