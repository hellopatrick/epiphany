use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
  Acc(i32),
  Jmp(i32),
  Noop(i32),
}

impl FromStr for Instruction {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut words = s.split_whitespace();

    let inst = words.next().unwrap();
    let val = words.next().unwrap().parse().unwrap();

    Ok(match inst {
      "jmp" => Instruction::Jmp(val),
      "nop" => Instruction::Noop(val),
      "acc" => Instruction::Acc(val),
      _ => unreachable!(),
    })
  }
}

#[derive(Default, Debug)]
pub struct Program {
  instructions: Vec<Instruction>,
  pc: usize,
  accumulator: i32,
  seen_pc: HashSet<usize>,
}

impl Program {
  fn run(&mut self) -> (i32, bool) {
    loop {
      if self.seen_pc.contains(&self.pc) {
        return (self.accumulator, false);
      }

      if self.pc >= self.instructions.len() {
        return (self.accumulator, true);
      }

      self.seen_pc.insert(self.pc);

      match self.instructions[self.pc] {
        Instruction::Acc(da) => {
          self.accumulator += da;
          self.pc += 1;
        }
        Instruction::Jmp(dc) => {
          self.pc = (self.pc as i32 + dc) as usize;
        }
        Instruction::Noop(_) => {
          self.pc += 1;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn input() -> Vec<Instruction> {
    include_str!("../data/08.txt")
      .lines()
      .map(|line| line.parse().unwrap())
      .collect()
  }

  #[test]
  fn part_01() {
    let mut prog = Program::default();

    prog.instructions = input();

    let (last_value, _) = prog.run();

    assert_eq!(last_value, 1563);
  }

  #[test]
  fn part_02() {
    let instructions = input();

    for i in 0..instructions.len() {
      let mut prog = Program::default();
      let mut new_instructions = instructions.clone();

      match new_instructions[i] {
        Instruction::Acc(_) => {
          continue;
        }
        Instruction::Jmp(c) => {
          new_instructions[i] = Instruction::Noop(c);
        }
        Instruction::Noop(c) => {
          new_instructions[i] = Instruction::Jmp(c);
        }
      }

      prog.instructions = new_instructions;

      let (last_value, completed) = prog.run();

      if completed {
        assert_eq!(last_value, 767);
      }
    }

    unreachable!();
  }
}
