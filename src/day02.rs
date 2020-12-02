use std::str::FromStr;

use regex::Regex;

struct PasswordPolicy {
  min: usize,
  max: usize,
  letter: char,
}

struct Password {
  policy: PasswordPolicy,
  pass: String,
}

impl FromStr for Password {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let pattern = Regex::new(r"(\d+)-(\d+)\s+(.):\s+(.+)").unwrap();

    let groups = match pattern.captures(s) {
      Some(g) => g,
      None => return Err("Unable to parse string".to_string()),
    };

    let min = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = groups.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let pass = groups.get(4).unwrap().as_str().to_string();

    let policy = PasswordPolicy { min, max, letter };

    Ok(Password { policy, pass })
  }
}

impl Password {
  fn is_valid_counts(&self) -> bool {
    let count = self
      .pass
      .chars()
      .filter(|&c| c == self.policy.letter)
      .count();

    self.policy.min <= count && count <= self.policy.max
  }

  fn is_valid_location(&self) -> bool {
    let chars: Vec<char> = self.pass.chars().collect();

    (chars[self.policy.min - 1] == self.policy.letter)
      ^ (chars[self.policy.max - 1] == self.policy.letter)
  }
}

pub fn first_star(input: &[&str]) -> usize {
  input
    .iter()
    .map(|&line| line.parse::<Password>().unwrap())
    .filter(|p| p.is_valid_counts())
    .count()
}

pub fn second_star(input: &[&str]) -> usize {
  input
    .iter()
    .map(|&line| line.parse::<Password>().unwrap())
    .filter(|p| p.is_valid_location())
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<&'static str> {
    include_str!("../data/02.txt").lines().collect()
  }

  #[test]
  fn part_01() {
    let lines = input();

    let solution = first_star(&lines);

    assert_eq!(solution, 517);
  }

  #[test]
  fn part_02() {
    let lines = input();

    let solution = second_star(&lines);

    assert_eq!(solution, 284);
  }
}
