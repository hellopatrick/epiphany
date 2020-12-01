pub fn first_star(input: &[usize]) -> Option<usize> {
  let len = input.len();
  for i in 0..len {
    for j in i + 1..len {
      let a = input[i];
      let b = input[j];

      if a + b == 2020 {
        return Some(a * b);
      }
    }
  }

  None
}

pub fn second_star(input: &[usize]) -> Option<usize> {
  let len = input.len();
  for i in 0..len {
    for j in i + 1..len {
      for k in j + 1..len {
        let a = input[i];
        let b = input[j];
        let c = input[k];

        if a + b + c == 2020 {
          return Some(a * b * c);
        }
      }
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::{first_star, second_star};

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
