use std::collections::HashSet;

#[inline]
fn uniq_chars(group: &str) -> HashSet<char> {
  group.chars().filter(|c| c.is_alphabetic()).collect()
}

fn all_yes(group: &str) -> HashSet<char> {
  let all: HashSet<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

  group
    .lines()
    .map(|line| uniq_chars(line))
    .fold(all, |acc, next| acc.intersection(&next).copied().collect())
}

pub fn first_star(groups: &[&str]) -> usize {
  groups.iter().map(|&group| uniq_chars(group).len()).sum()
}

pub fn second_star(groups: &[&str]) -> usize {
  groups.iter().map(|&group| all_yes(group).len()).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<&'static str> {
    include_str!("../data/06.txt").split("\n\n").collect()
  }

  #[test]
  fn part_01() {
    let groups = input();

    let answer = first_star(&groups);

    assert_eq!(answer, 6521);
  }

  #[test]
  fn part_02() {
    let groups = input();

    let answer = second_star(&groups);

    assert_eq!(answer, 3305);
  }
}
