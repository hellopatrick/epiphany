pub fn translate(seat_id: &str) -> usize {
  seat_id
    .chars()
    .map(|c| match c {
      'F' => 0,
      'B' => 1,
      'L' => 0,
      'R' => 1,
      _ => unreachable!(),
    })
    .rev()
    .enumerate()
    .fold(0, |total, (i, bit)| total + (bit << i))
}

pub fn first_star(seat_ids: &[&str]) -> usize {
  seat_ids
    .iter()
    .map(|&seat_id| translate(seat_id))
    .max()
    .expect("no max found")
}

pub fn second_star(seat_ids: &[&str]) -> usize {
  let mut seat_ids: Vec<_> = seat_ids.iter().map(|&seat_id| translate(seat_id)).collect();

  seat_ids.sort();

  let res = seat_ids
    .windows(2)
    .find(|&pair| match pair {
      [a, b] => *a + 1 != *b,
      _ => unreachable!(),
    })
    .expect("no missing seat found");

  res[0] + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<&'static str> {
    include_str!("../data/05.txt").lines().collect()
  }

  #[test]
  fn part_01() {
    let seat_ids = input();

    let answer = first_star(&seat_ids);

    assert_eq!(answer, 970);
  }

  #[test]
  fn part_02() {
    let seat_ids = input();

    let answer = second_star(&seat_ids);

    assert_eq!(answer, 587);
  }
}
