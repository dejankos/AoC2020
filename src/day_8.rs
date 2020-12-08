use std::collections::HashSet;

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
    res: usize,
    loop_detected: bool,
}

impl Console {
    fn new(instructions: Vec<Instruction>) -> Self {
        Console {
            instructions,
            acc: vec![0],
            instructions_history: HashSet::new(),
            next_instructions: 0,
            res: 0,
            loop_detected: false,
        }
    }

    fn run(&mut self) -> usize {
        loop {
            if let Some(inst) = self.instructions.get(self.next_instructions) {
                if self.loop_detected(inst) {
                    self.res = self.acc[self.acc.len() - 2] as usize;
                    self.loop_detected = true;
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
            } else {
                self.res = self.acc.last().copied().unwrap() as usize;
                break;
            }
        }

        self.res
    }

    fn run_part_2(&mut self) -> usize {
        let orig_inst = self.instructions.clone();
        for (i, inst) in orig_inst.iter().enumerate() {
            match inst.op {
                Op::Nop(v) | Op::Jmp(v) => {
                    let mut new_inst = orig_inst.clone();
                    let m_inst = new_inst.get_mut(i).unwrap();
                    m_inst.op = if matches!(inst.op, Op::Jmp(_)) {
                        Op::Nop(v)
                    } else {
                        Op::Jmp(v)
                    };
                    self.instructions = new_inst;
                    self.reset();

                    self.run();

                    if !self.loop_detected {
                        break;
                    }
                }
                _ => {}
            }
        }

        self.res
    }

    fn reset(&mut self) {
        self.loop_detected = false;
        self.acc = vec![0];
        self.res = 0;
        self.instructions_history = HashSet::new();
        self.next_instructions = 0;
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

        assert_eq!(5, c.run());
    }

    #[test]
    fn should_solve_part_1() {
        let inst = parse_data(parse_lines("input/day_8_data.txt"));
        let mut c = Console::new(inst);
        assert_eq!(1489, c.run());
    }

    #[test]
    fn should_solve_part_2() {
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

        assert_eq!(8, c.run_part_2());
    }

    #[test]
    fn should_solve_part_2_day_data() {
        let inst = parse_data(parse_lines("input/day_8_data.txt"));
        let mut c = Console::new(inst);
        assert_eq!(1539, c.run_part_2());
    }
}
