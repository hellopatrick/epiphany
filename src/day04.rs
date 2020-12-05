use std::{collections::HashMap, str::FromStr};

use regex::Regex;

pub struct Passport {
  byr: Option<usize>,
  iyr: Option<usize>,
  eyr: Option<usize>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  _cid: Option<String>,
}

impl Passport {
  pub fn has_data(&self) -> bool {
    self.byr.is_some()
      && self.iyr.is_some()
      && self.eyr.is_some()
      && self.hgt.is_some()
      && self.hcl.is_some()
      && self.ecl.is_some()
      && self.pid.is_some()
  }

  pub fn is_valid(&self) -> bool {
    let r = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let s = Regex::new(r"^[0-9]{9}$").unwrap();
    let t = Regex::new(r"^(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$").unwrap();

    self.byr.filter(|&c| 1920 <= c && c <= 2002).is_some()
      && self.iyr.filter(|&c| 2010 <= c && c <= 2020).is_some()
      && self.eyr.filter(|&c| 2020 <= c && c <= 2030).is_some()
      && self.hgt.clone().filter(|c| t.is_match(c)).is_some()
      && self.hcl.clone().filter(|c| r.is_match(c)).is_some()
      && self
        .ecl
        .clone()
        .filter(|c| {
          [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
          ]
          .contains(c)
        })
        .is_some()
      && self.pid.clone().filter(|c| s.is_match(c)).is_some()
  }
}

impl FromStr for Passport {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let kv: HashMap<_, _> = s
      .split_whitespace()
      .map(|kv| {
        let mut sp = kv.split(":");
        (sp.next().unwrap(), sp.next().unwrap().to_string())
      })
      .collect();

    Ok(Passport {
      byr: kv.get("byr").map(|c| c.parse().unwrap()),
      iyr: kv.get("iyr").map(|c| c.parse().unwrap()),
      eyr: kv.get("eyr").map(|c| c.parse().unwrap()),
      hgt: kv.get("hgt").cloned(),
      hcl: kv.get("hcl").cloned(),
      ecl: kv.get("ecl").cloned(),
      pid: kv.get("pid").cloned(),
      _cid: kv.get("cid").cloned(),
    })
  }
}

pub fn first_star(passports: &[Passport]) -> usize {
  passports.iter().filter(|&p| p.has_data()).count()
}

pub fn second_star(passports: &[Passport]) -> usize {
  passports.iter().filter(|&p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<Passport> {
    include_str!("../data/04.txt")
      .split("\n\n")
      .map(|l| l.parse().unwrap())
      .collect()
  }

  #[test]
  fn part_01() {
    let passports = input();

    let valid_passport_count = first_star(&passports);

    assert_eq!(valid_passport_count, 233);
  }

  #[test]
  fn part_02() {
    let passports = input();

    let valid_passport_count = second_star(&passports);

    assert_eq!(valid_passport_count, 111);
  }
}
