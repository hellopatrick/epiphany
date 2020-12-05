pub fn translate(seat_id: &str) -> (usize, usize) {
  let row = seat_id.get(0..7).unwrap();
  let col = seat_id.get(7..10).unwrap();

  let row = row.replace("F", "0").replace("B", "1");
  let col = col.replace("L", "0").replace("R", "1");

  (
    usize::from_str_radix(&row, 2).unwrap(),
    usize::from_str_radix(&col, 2).unwrap(),
  )
}

pub fn first_star(seat_ids: &[&str]) -> usize {
  seat_ids
    .iter()
    .map(|&seat_id| translate(seat_id))
    .map(|(a, b)| a * 8 + b)
    .max()
    .unwrap()
}

pub fn second_star(seat_ids: &[&str]) -> usize {
  let mut seat_ids: Vec<_> = seat_ids
    .iter()
    .map(|&seat_id| translate(seat_id))
    .map(|(a, b)| a * 8 + b)
    .collect();

  seat_ids.sort();

  let res = seat_ids
    .windows(2)
    .find(|&pair| {
      if let [a, b] = pair {
        *a + 1 != *b
      } else {
        false
      }
    })
    .unwrap();

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
