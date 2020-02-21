use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

fn main() {
  let instructions = read_input("input.txt");
  let mut computer = Computer::new(&instructions);
  computer.run();
  println!(
    "Day 23: Opening the Turing Lock part1 solution\n {}",
    *computer.registers.get(&Register::B).unwrap()
  );

  computer = Computer::new(&instructions);
  computer.registers.insert(Register::A, 1);
  computer.run();
  println!(
    "Day 23: Opening the Turing Lock part2 solution\n {}",
    *computer.registers.get(&Register::B).unwrap()
  );
}

struct Computer {
  registers: HashMap<Register, usize>,
  instructions: Vec<Instruction>,
  pos: i32,
}
impl Computer {
  pub fn new(instructions: &Vec<Instruction>) -> Computer {
    let mut registers = HashMap::new();
    registers.insert(Register::A, 0);
    registers.insert(Register::B, 0);
    Computer {
      registers,
      instructions: instructions.clone(),
      pos: 0,
    }
  }

  pub fn run(&mut self) {
    loop {
      let curr_instruction = self.instructions[self.pos as usize];
      match curr_instruction.command {
        Command::Hlf => {
          let value = self
            .registers
            .get_mut(&curr_instruction.register.unwrap())
            .unwrap();
          *value /= 2;
        }
        Command::Tpl => {
          let value = self
            .registers
            .get_mut(&curr_instruction.register.unwrap())
            .unwrap();
          *value *= 3;
        }
        Command::Inc => {
          let value = self
            .registers
            .get_mut(&curr_instruction.register.unwrap())
            .unwrap();
          *value += 1;
        }
        Command::Jmp => {
          let offset = &curr_instruction.offset.unwrap();
          self.pos += offset;
          continue;
        }
        Command::Jie => {
          let offset = &curr_instruction.offset.unwrap();
          let value = self
            .registers
            .get(&curr_instruction.register.unwrap())
            .unwrap();
          if *value % 2 == 0 {
            self.pos += offset;
            continue;
          }
        }
        Command::Jio => {
          let offset = &curr_instruction.offset.unwrap();
          let value = self
            .registers
            .get(&curr_instruction.register.unwrap())
            .unwrap();
          if *value == 1 {
            self.pos += offset;
            if self.pos < 0 || self.pos >= self.instructions.len() as i32 {
              return;
            }
            continue;
          }
        }
      }
      self.pos += 1;
      if self.pos < 0 || self.pos >= self.instructions.len() as i32 {
        return;
      }
    }
  }
}

#[derive(Clone, Copy)]
struct Instruction {
  command: Command,
  register: Option<Register>,
  offset: Option<i32>,
}
impl Instruction {
  pub fn new(
    command: Command,
    register: Option<Register>,
    offset: Option<i32>,
  ) -> Instruction {
    Instruction {
      command,
      register,
      offset,
    }
  }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Register {
  A,
  B,
}

#[derive(Copy, Clone, PartialEq)]
enum Command {
  Hlf,
  Tpl,
  Inc,
  Jmp,
  Jie,
  Jio,
}

fn read_input(filename: &str) -> Vec<Instruction> {
  let mut file = File::open(filename).expect("File not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read input file");
  let mut instructions = Vec::new();
  let re = Regex::new(r"(.+?)\s(a|b)?,?\s?\+?(-?\d+)?").unwrap();
  for s in contents.split_terminator('\n') {
    let captures = re.captures(s).unwrap();
    let command = match &captures[1] {
      "hlf" => Command::Hlf,
      "tpl" => Command::Tpl,
      "inc" => Command::Inc,
      "jmp" => Command::Jmp,
      "jie" => Command::Jie,
      "jio" => Command::Jio,
      _ => panic!("Unknown command {}", &captures[1]),
    };
    let mut offset = None;
    let mut register = None;
    if let Some(reg) = captures.get(2) {
      register = match reg.as_str() {
        "a" => Some(Register::A),
        "b" => Some(Register::B),
        _ => panic!("Unknown register {}", &captures[2]),
      }
    }
    if captures.get(3).is_some() {
      offset = Some(captures[3].parse::<i32>().unwrap());
    }
    instructions.push(Instruction::new(command, register, offset));
  }

  instructions
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let instructions = read_input("test-input.txt");
    let mut computer = Computer::new(&instructions);
    computer.run();
    assert_eq!(*computer.registers.get(&Register::A).unwrap(), 2);
  }
}
