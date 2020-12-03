pub fn trees_hit(pattern: &[&str], slope: (usize, usize)) -> usize {
  let (dy, dx) = slope;

  pattern
    .iter()
    .step_by(dy)
    .enumerate()
    .map(|(i, &line)| {
      let x = dx * i % line.len();
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
  trees_hit(pattern, (3, 1))
}

pub fn second_star(pattern: &[&str]) -> usize {
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|&slope| trees_hit(pattern, slope))
    .product()
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
