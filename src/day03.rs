pub fn trees_hit(pattern: &[&str], slope_right: usize, slope_down: usize) -> usize {
  pattern
    .iter()
    .step_by(slope_down)
    .enumerate()
    .map(|(i, &line)| {
      let x = slope_right * i % line.len();
      let c = line.chars().nth(x).unwrap();

      if c == '.' {
        0
      } else {
        1
      }
    })
    .sum()
}

pub fn first_star(pattern: &[&str]) -> usize {
  trees_hit(pattern, 3, 1)
}

pub fn second_star(pattern: &[&str]) -> usize {
  trees_hit(pattern, 1, 1)
    * trees_hit(pattern, 3, 1)
    * trees_hit(pattern, 5, 1)
    * trees_hit(pattern, 7, 1)
    * trees_hit(pattern, 1, 2)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<&'static str> {
    include_str!("../data/03.txt").lines().collect()
  }

  #[test]
  fn part_01() {
    let pattern = input();

    let trees = first_star(&pattern);

    assert_eq!(trees, 230);
  }

  #[test]
  fn part_02() {
    let pattern = input();

    let trees = second_star(&pattern);

    assert_eq!(trees, 9533698720);
  }
}
