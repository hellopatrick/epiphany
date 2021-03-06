use itertools::Itertools;

const DESIRED_SUM: usize = 2020;

fn solution(input: &[usize], length: usize) -> Option<usize> {
  input
    .iter()
    .combinations(length)
    .find(|combo| combo.iter().copied().sum::<usize>() == DESIRED_SUM)
    .map(|combo| combo.iter().copied().product::<usize>())
}

pub fn first_star(input: &[usize]) -> Option<usize> {
  solution(input, 2)
}

pub fn second_star(input: &[usize]) -> Option<usize> {
  solution(input, 3)
}

#[cfg(test)]
mod tests {
  use super::*;

  const fn sample_data() -> [usize; 6] {
    return [1721, 979, 366, 299, 675, 1456];
  }

  fn puzzle_data() -> Vec<usize> {
    return include_str!("../data/01.txt")
      .split_whitespace()
      .map(|l| l.parse().unwrap())
      .collect();
  }

  #[test]
  fn part_01_sample() {
    let input = sample_data();
    let res = first_star(&input);

    assert_eq!(res, Some(514579));
  }

  #[test]
  fn part_01() {
    let input = puzzle_data();
    let res = first_star(&input);

    assert_eq!(res, Some(445536));
  }

  #[test]
  fn part_02_sample() {
    let input = sample_data();
    let res = second_star(&input);

    assert_eq!(res, Some(241861950));
  }

  #[test]
  fn part_02() {
    let input = puzzle_data();
    let res = second_star(&input);

    assert_eq!(res, Some(138688160));
  }
}
