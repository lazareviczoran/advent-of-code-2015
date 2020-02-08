use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");

  let strings: Vec<&str> = contents.split_terminator('\n').collect();

  let mut total_nice_strings_count = 0;
  for s in &strings {
    if is_nice(s) {
      total_nice_strings_count += 1;
    }
  }

  println!(
    "Day 5: Doesn't He Have Intern-Elves For This? part1 solution:\n {}",
    total_nice_strings_count
  );

  total_nice_strings_count = 0;
  for s in strings {
    if is_nice_v2(s) {
      total_nice_strings_count += 1;
    }
  }
  println!(
    "Day 5: Doesn't He Have Intern-Elves For This? part2 solution:\n {}",
    total_nice_strings_count
  );
}

fn is_nice(string: &str) -> bool {
  let mut vowel_count = 0;
  let mut has_repeated_adjs = false;
  let mut has_disallowed_adjs = false;
  let mut chars = string.chars().peekable();
  while let Some(ch) = chars.next() {
    if "aeiou".contains(ch) {
      vowel_count += 1;
    }
    if let Some(next) = chars.peek() {
      if !has_repeated_adjs && ch == *next {
        has_repeated_adjs = true;
      }
      let mut adjs_string = String::new();
      adjs_string.push(ch);
      adjs_string.push(*next);
      if "ab,cd,pq,xy".contains(&adjs_string.as_str()) {
        has_disallowed_adjs = true;
      }
    }
  }

  vowel_count >= 3 && has_repeated_adjs && !has_disallowed_adjs
}

fn is_nice_v2(string: &str) -> bool {
  let mut has_repeated_pair = false;
  let mut has_mirrored_seq = false;
  let mut chars = string.chars();
  let mut prev = None;
  while let Some(ch) = chars.next() {
    let mut peekable = chars.clone().peekable();
    if let Some(next_ch) = peekable.peek() {
      if !has_repeated_pair {
        let (_, remaining) = chars.as_str().split_at(1);
        let mut pair = String::new();
        pair.push(ch);
        pair.push(*next_ch);
        if remaining.contains(&pair) {
          has_repeated_pair = true;
        }
      }
      if !has_mirrored_seq && prev.is_some() {
        if prev.unwrap() == *next_ch {
          has_mirrored_seq = true;
        }
      }
    }
    prev = Some(ch);
  }

  has_mirrored_seq && has_repeated_pair
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_input1() {
    assert_eq!(is_nice("ugknbfddgicrmopn"), true);
  }

  #[test]
  fn part1_input2() {
    assert_eq!(is_nice("aaa"), true);
  }

  #[test]
  fn part1_input3() {
    assert_eq!(is_nice("jchzalrnumimnmhp"), false);
  }

  #[test]
  fn part1_input4() {
    assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
  }

  #[test]
  fn part1_input5() {
    assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
  }

  #[test]
  fn part2_input1() {
    assert_eq!(is_nice_v2("qjhvhtzxzqqjkmpb"), true);
  }

  #[test]
  fn part2_input2() {
    assert_eq!(is_nice_v2("xxyxx"), true);
  }

  #[test]
  fn part2_input3() {
    assert_eq!(is_nice_v2("uurcxstgmygtbstg"), false);
  }

  #[test]
  fn part2_input4() {
    assert_eq!(is_nice_v2("ieodomkazucvgmuy"), false);
  }
}
