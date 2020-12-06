use std::collections::HashMap;

fn yes_answers(group: &str) -> HashMap<char, usize> {
  let mut map = HashMap::with_capacity(group.len());

  for c in group.chars().filter(|c| c.is_alphabetic()) {
    let counter = map.entry(c).or_insert(0);
    *counter += 1;
  }

  map
}

pub fn first_star(groups: &[&str]) -> usize {
  groups.iter().map(|&group| yes_answers(group).len()).sum()
}

pub fn second_star(groups: &[&str]) -> usize {
  groups
    .iter()
    .map(|&group| {
      let members = group.lines().count();

      let answer_counts = yes_answers(group);

      answer_counts.iter().filter(|(_, &b)| b == members).count()
    })
    .sum()
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
