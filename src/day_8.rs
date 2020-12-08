use std::collections::{HashSet};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Instruction {
    idx: usize,
    op: Op,
}

impl Instruction {
    fn new(idx: usize, op: Op) -> Self {
        Instruction { idx, op }
    }
}

struct Console {
    instructions: Vec<Instruction>,
    acc: Vec<isize>,
    instructions_history: HashSet<Instruction>,
    next_instructions: usize,
}

impl Console {
    fn new(instructions: Vec<Instruction>) -> Self {
        Console {
            instructions,
            acc: vec![0],
            instructions_history: HashSet::new(),
            next_instructions: 0,
        }
    }

    fn run(&mut self) -> usize {
        loop {
            if let Some(inst) = self.instructions.get(self.next_instructions) {
                if self.loop_detected(inst) {
                    break;
                }

                match inst.op {
                    Op::Acc(v) => {
                        self.acc.push(self.acc.last().unwrap() + v);
                        self.next_instructions += 1;
                        self.instructions_history.insert(*inst);
                    }
                    Op::Jmp(v) => {
                        self.acc.push(*self.acc.last().unwrap());
                        self.next_instructions = ((self.next_instructions as isize) + v) as usize;
                        self.instructions_history.insert(*inst);
                    }
                    Op::Nop(_) => {
                        self.acc.push(*self.acc.last().unwrap());
                        self.next_instructions += 1;
                        self.instructions_history.insert(*inst);
                    }
                }
            }
        }

        self.acc[self.acc.len() - 2] as usize
    }

    fn loop_detected(&self, inst: &Instruction) -> bool {
        self.instructions_history.contains(inst)
    }
}

fn parse_data(data: Vec<String>) -> Vec<Instruction> {
    data.into_iter()
        .enumerate()
        .map(|(i, l)| {
            let s = l.split_whitespace().collect_vec();
            let arg = s[1].parse::<isize>().expect("arg should have a value");
            match s[0] {
                "acc" => Instruction::new(i, Op::Acc(arg)),
                "jmp" => Instruction::new(i, Op::Jmp(arg)),
                "nop" => Instruction::new(i, Op::Nop(arg)),
                _ => panic!("unknown op"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

    #[test]
    fn should_solve() {
        let data = vec![
            "nop +0".into(),
            "acc +1".into(),
            "jmp +4".into(),
            "acc +3".into(),
            "jmp -3".into(),
            "acc -99".into(),
            "acc +1".into(),
            "jmp -4".into(),
            "acc +6".into(),
        ];

        let inst = parse_data(data);
        let mut c = Console::new(inst);

        println!("{}", c.run());
    }

    #[test]
    fn should_solve_part_1() {
        let inst = parse_data(parse_lines("input/day_8_data.txt"));
        let mut c = Console::new(inst);
        assert_eq!(1489, c.run());
    }
}
