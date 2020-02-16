fn main() {
  let input = 36000000;

  println!(
    "Day 20: Infinite Elves and Infinite Houses part1 solution\n {:?}",
    find_first_house_with_at_least_n_presents(input)
  );

  println!(
    "Day 20: Infinite Elves and Infinite Houses part2 solution\n {:?}",
    find_first_house_with_at_least_n_presents_p2(input)
  );
}

fn find_first_house_with_at_least_n_presents(n: usize) -> usize {
  let limit = n / 10;
  let mut houses = vec![0; limit];
  for elf in 1..limit {
    let mut house = elf;
    while house < limit {
      houses[house] += elf * 10;
      house += elf;
    }
  }

  houses.into_iter().position(|p| p >= n).unwrap()
}

fn find_first_house_with_at_least_n_presents_p2(n: usize) -> usize {
  let limit = n / 11;
  let mut houses = vec![0; limit];
  for elf in 1..n {
    let mut house = elf;
    let mut count = 0;
    while house < limit && count < 50 {
      houses[house] += elf * 11;
      house += elf;
      count += 1;
    }
  }

  println!("{:?}", &houses[0..10]);

  houses.into_iter().position(|p| p >= n).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_find_first_house_with_at_least_n_presents() {
    assert_eq!(find_first_house_with_at_least_n_presents(130), 8);
    assert_eq!(find_first_house_with_at_least_n_presents(60), 4);
    assert_eq!(find_first_house_with_at_least_n_presents(120), 6);
    assert_eq!(find_first_house_with_at_least_n_presents(80), 6);
    assert_eq!(find_first_house_with_at_least_n_presents(125), 8);
  }
}
