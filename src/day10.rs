pub fn first_star(input: &[bool]) -> usize {
  let init = (0, 0);

  let res = input.windows(2).fold(init, |acc, b| {
    let first = b[0];
    let second = b[1];

    if first {
      if second {
        // if next exist, then diff is 1
        (acc.0 + 1, acc.1)
      } else {
        // otherwise we're guaranteed that 3 is away
        (acc.0, acc.1 + 1)
      }
    } else {
      acc
    }
  });

  res.0 * res.1
}

pub fn second_star(input: &[bool]) -> usize {
  let mut counts: Vec<usize> = vec![0; input.len()];

  // pre-computed based on input to make the next for loop simple.
  counts[0] = 1;
  counts[1] = 1;
  counts[2] = 2;
  counts[3] = 4;

  for (i, &exists) in input.iter().enumerate().skip(4) {
    if exists {
      counts[i] = counts[i - 1] + counts[i - 2] + counts[i - 3];
    }
  }

  counts.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<bool> {
    let data: Vec<usize> = include_str!("../data/10.txt")
      .lines()
      .map(|c| c.parse().unwrap())
      .collect();

    let max = *data.iter().max().unwrap();

    let mut adapters = vec![false; max + 4];

    adapters[0] = true;
    adapters[max + 3] = true;

    for b in data {
      adapters[b] = true;
    }

    adapters
  }

  #[test]
  fn part_01() {
    let input = input();

    let actual = first_star(&input);

    assert_eq!(actual, 1656);
  }

  #[test]
  fn part_02() {
    let input = input();
    let actual = second_star(&input);

    assert_eq!(actual, 56693912375296);
  }
}
