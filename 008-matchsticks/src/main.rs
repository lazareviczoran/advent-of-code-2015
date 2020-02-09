use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");

  let strings = contents
    .split_terminator('\n')
    .map(|s| format!("{}", s))
    .collect();
  println!(
    "Day 8: Matchsticks part1 solution:\n {}",
    calculate_total_decoded_diff(&strings)
  );

  println!(
    "Day 8: Matchsticks part2 solution:\n {}",
    calculate_total_encoded_diff(&strings)
  );
}

fn calculate_total_decoded_diff(strings: &Vec<String>) -> usize {
  let mut total_unescaped_length = 0;
  let mut total_escaped_length = 0;
  for s in strings {
    total_unescaped_length += s.len();
    total_escaped_length += get_decoded_string_length(s);
  }

  total_unescaped_length - total_escaped_length
}

fn calculate_total_encoded_diff(strings: &Vec<String>) -> usize {
  let mut total_unescaped_length = 0;
  let mut total_escaped_length = 0;
  for s in strings {
    total_escaped_length += s.len();
    total_unescaped_length += get_encoded_string_length(s);
  }

  total_unescaped_length - total_escaped_length
}

fn get_encoded_string_length(string: &str) -> usize {
  let mut res = String::from("\"");
  let mut chars = string.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '\\' || ch == '"' {
      res.push('\\');
    }
    res.push(ch);
  }
  res.push('"');
  res.len()
}

fn get_decoded_string_length(string: &str) -> usize {
  let mut count = 0;
  let mut res = String::new();
  let stripped_string = &string[1..string.len() - 1];
  let mut chars = stripped_string.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '\\' {
      if let Some(next_ch) = chars.peek() {
        match next_ch {
          '\\' | '"' => {
            res.push(*next_ch);
            count += 1;
            chars.next();
          }
          'x' => {
            chars.next();
            let mut hex = String::new();
            hex.push(chars.next().unwrap());
            hex.push(chars.next().unwrap());

            let value = i64::from_str_radix(&hex, 16).unwrap();
            res.push(value as u8 as char);
            count += 1;
          }
          _ => {
            res.push(ch);
            count += 1;
          }
        }
      }
    } else {
      res.push(ch);
      count += 1;
    }
  }

  count
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_input1() {
    let mut strings = Vec::new();
    strings.push(format!("{}", r#""""#));
    strings.push(format!("{}", r#""abc""#));
    strings.push(format!("{}", r#""aaa\"aaa""#));
    strings.push(format!("{}", r#""\x27""#));
    assert_eq!(calculate_total_decoded_diff(&strings), 12);
  }

  #[test]
  fn part2_input1() {
    let mut strings = Vec::new();
    strings.push(format!("{}", r#""""#));
    strings.push(format!("{}", r#""abc""#));
    strings.push(format!("{}", r#""aaa\"aaa""#));
    strings.push(format!("{}", r#""\x27""#));
    assert_eq!(calculate_total_encoded_diff(&strings), 19);
  }
}
