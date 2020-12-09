use itertools::Itertools;

fn sum_exists(target: usize, prefix: &[usize]) -> bool {
  prefix
    .iter()
    .combinations(2)
    .find(|comb| comb[0] + comb[1] == target)
    .is_some()
}

pub fn first_star(values: &[usize]) -> usize {
  values
    .windows(26)
    .find(|&window| {
      let prefix = &window[0..25];
      let target = window[25];
      !sum_exists(target, prefix)
    })
    .and_then(|window| window.last().copied())
    .unwrap()
}

pub fn second_star(values: &[usize], target: usize) -> usize {
  for i in 0..values.len() {
    for j in i + 1..values.len() {
      let window = &values[i..=j];
      let total: usize = window.iter().sum();

      if total == target {
        return window.iter().max().unwrap() + window.iter().min().unwrap();
      } else if total > target {
        break;
      }
    }
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<usize> {
    include_str!("../data/09.txt")
      .lines()
      .map(|line| line.parse().unwrap())
      .collect()
  }

  #[test]
  fn part_01() {
    let values = input();

    let actual = first_star(&values);

    assert_eq!(actual, 1398413738);
  }

  #[test]
  fn part_02() {
    let values = input();

    let actual = second_star(&values, 1398413738);

    assert_eq!(actual, 169521051);
  }
}
