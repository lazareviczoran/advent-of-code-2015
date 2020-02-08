use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");
  let presents: Vec<Prism> = contents
    .split_terminator('\n')
    .map(|v| {
      let dimensions = v
        .split_terminator('x')
        .map(|p| p.parse::<usize>().unwrap())
        .collect();
      Prism::new(dimensions)
    })
    .collect();

  println!(
    "Day 2: I Was Told There Would Be No Math part1 solution:\n {}",
    presents
      .iter()
      .fold(0, |acc, p| acc + p.get_required_wrapping_paper_amount())
  );

  println!(
    "Day 2: I Was Told There Would Be No Math part2 solution:\n {}",
    presents
      .iter()
      .fold(0, |acc, p| acc + p.get_required_ribbon_amount())
  );
}

struct Prism {
  l: usize,
  w: usize,
  h: usize,
}
impl Prism {
  pub fn new(dimensions: Vec<usize>) -> Prism {
    Prism {
      l: dimensions[0],
      w: dimensions[1],
      h: dimensions[2],
    }
  }

  pub fn get_required_wrapping_paper_amount(&self) -> usize {
    let surface_areas = vec![self.l * self.w, self.w * self.h, self.h * self.l];
    let smallest_area =
      min(min(surface_areas[0], surface_areas[1]), surface_areas[2]);

    surface_areas.iter().fold(0, |acc, area| acc + 2 * area) + smallest_area
  }

  pub fn get_required_ribbon_amount(&self) -> usize {
    let mut dimensions = vec![self.l, self.w, self.h];
    dimensions.sort();

    2 * dimensions[0]
      + 2 * dimensions[1]
      + dimensions.iter().fold(1, |acc, d| acc * d)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_input1() {
    let present = Prism::new(vec![2, 3, 4]);
    assert_eq!(present.get_required_wrapping_paper_amount(), 58);
  }

  #[test]
  fn part1_input2() {
    let present = Prism::new(vec![1, 1, 10]);
    assert_eq!(present.get_required_wrapping_paper_amount(), 43);
  }

  #[test]
  fn part2_input1() {
    let present = Prism::new(vec![2, 3, 4]);
    assert_eq!(present.get_required_ribbon_amount(), 34);
  }

  #[test]
  fn part2_input2() {
    let present = Prism::new(vec![1, 1, 10]);
    assert_eq!(present.get_required_ribbon_amount(), 14);
  }
}
