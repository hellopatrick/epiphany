pub fn trees_hit(pattern: &[Vec<bool>], slope: (usize, usize)) -> usize {
  let (dx, dy) = slope;

  pattern
    .iter()
    .step_by(dy)
    .enumerate()
    .filter(|(i, line)| {
      let x = dx * i % line.len();
      line[x]
    })
    .count()
}

pub fn first_star(pattern: &[Vec<bool>]) -> usize {
  trees_hit(pattern, (3, 1))
}

pub fn second_star(pattern: &[Vec<bool>]) -> usize {
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|&slope| trees_hit(pattern, slope))
    .product()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<Vec<bool>> {
    include_str!("../data/03.txt")
      .lines()
      .map(|line| line.chars().map(|c| c == '#').collect())
      .collect()
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
