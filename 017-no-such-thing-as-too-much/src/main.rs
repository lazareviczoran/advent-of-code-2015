use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let containers = read_input("input.txt");
  println!(
    "Day 17: No Such Thing as Too Much part1 solution\n {}",
    find_all_container_combinations(&containers, &vec![], 0, 150).len()
  );

  println!(
    "Day 17: No Such Thing as Too Much part2 solution\n {}",
    find_minimum_container_combinations(&containers, 150)
  );
}

fn find_minimum_container_combinations(
  containers: &Vec<usize>,
  target_value: usize,
) -> usize {
  let combinations =
    find_all_container_combinations(containers, &vec![], 0, target_value);
  let mut min_combination_size = usize::max_value();
  let mut combinations_lenghts = HashMap::new();
  for c in combinations {
    if c.len() < min_combination_size {
      min_combination_size = c.len();
    }
    let count = combinations_lenghts.get(&c.len()).unwrap_or(&0);
    combinations_lenghts.insert(c.len(), count + 1);
  }

  *combinations_lenghts.get(&min_combination_size).unwrap()
}

fn find_all_container_combinations(
  containers: &Vec<usize>,
  current_set: &Vec<usize>,
  current_el_index: usize,
  target_value: usize,
) -> Vec<Vec<usize>> {
  let current_sum = current_set.iter().fold(0, |acc, x| acc + x);
  if current_sum == target_value {
    return vec![current_set.to_vec()];
  } else if current_sum > target_value || current_el_index == containers.len() {
    return vec![];
  }
  let mut current_set_with_curr_el = current_set.clone();
  current_set_with_curr_el.push(containers[current_el_index]);
  let mut combinations_with_curr_el = find_all_container_combinations(
    containers,
    &current_set_with_curr_el,
    current_el_index + 1,
    target_value,
  );
  let mut combinations_without_curr_el = find_all_container_combinations(
    containers,
    current_set,
    current_el_index + 1,
    target_value,
  );

  let mut res = Vec::new();
  res.append(&mut combinations_with_curr_el);
  res.append(&mut combinations_without_curr_el);

  res
}

fn read_input(filename: &str) -> Vec<usize> {
  let mut file = File::open(filename).expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");
  contents
    .split_terminator('\n')
    .map(|x| x.parse::<usize>().unwrap())
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_input1() {
    let containers = read_input("test-input.txt");
    assert_eq!(
      find_all_container_combinations(&containers, &vec![], 0, 25).len(),
      4
    );
  }

  #[test]
  fn part2_input1() {
    let containers = read_input("test-input.txt");
    assert_eq!(find_minimum_container_combinations(&containers, 25), 3);
  }
}
