use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");

  println!(
    "Day 1: Not Quite Lisp part1 solution:\n {}",
    calculate_floor(&contents)
  );

  println!(
    "Day 1: Not Quite Lisp part2 solution:\n {}",
    get_basement_first_enter_position(&contents)
  );
}

fn calculate_floor(input: &String) -> i32 {
  let mut current_floor = 0;
  let chars = input.chars();

  for ch in chars {
    match ch {
      '(' => current_floor += 1,
      ')' => current_floor -= 1,
      _ => panic!("Unexpected char {}", ch),
    }
  }

  current_floor
}

fn get_basement_first_enter_position(input: &String) -> usize {
  let mut current_floor = 0;
  let mut current_pos = 1;
  let chars = input.chars();

  for ch in chars {
    match ch {
      '(' => current_floor += 1,
      ')' => current_floor -= 1,
      _ => panic!("Unexpected char {}", ch),
    }
    if current_floor == -1 {
      return current_pos;
    }
    current_pos += 1;
  }

  current_pos
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_input1() {
    assert_eq!(calculate_floor(&String::from("(())")), 0);
    assert_eq!(calculate_floor(&String::from("()()")), 0);
  }

  #[test]
  fn part1_input2() {
    assert_eq!(calculate_floor(&String::from("(((")), 3);
    assert_eq!(calculate_floor(&String::from("(()(()(")), 3);
    assert_eq!(calculate_floor(&String::from("))(((((")), 3);
  }

  #[test]
  fn part1_input3() {
    assert_eq!(calculate_floor(&String::from("())")), -1);
    assert_eq!(calculate_floor(&String::from("))(")), -1);
  }

  #[test]
  fn part1_input4() {
    assert_eq!(calculate_floor(&String::from(")))")), -3);
    assert_eq!(calculate_floor(&String::from(")())())")), -3);
  }

  #[test]
  fn part2_input1() {
    assert_eq!(get_basement_first_enter_position(&String::from(")")), 1);
    assert_eq!(get_basement_first_enter_position(&String::from("()())")), 5);
  }
}
