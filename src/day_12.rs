type Position = (isize, isize);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn solve(inst: Vec<(char, isize)>) -> usize {
    let (mut p, mut d) = ((0, 0), Direction::E);
    let dirs = vec![Direction::N, Direction::E, Direction::S, Direction::W];

    inst.into_iter().for_each(|(ins, v)| match ins {
        'F' => {
            p = move_(&d, p, v);
        }
        'L' => {
            let idx = dirs.iter().position(|e| e == &d).unwrap() as isize - v / 90;
            d = if idx >= 0 {
                dirs[idx as usize]
            } else {
                dirs[4 - idx.abs() as usize]
            }
        }
        'R' => {
            let idx = dirs.iter().position(|e| e == &d).unwrap() as isize + v / 90;
            d = if idx < 4 {
                dirs[idx as usize]
            } else {
                dirs[idx as usize - 4]
            }
        }
        'N' => p = move_(&Direction::N, p, v),
        'E' => p = move_(&Direction::E, p, v),
        'W' => p = move_(&Direction::W, p, v),
        'S' => p = move_(&Direction::S, p, v),
        _ => {}
    });

    (p.0.abs() + p.1.abs()) as usize
}

fn solve_2(inst: Vec<(char, isize)>) -> usize {
    let (mut p, _d, mut wp) = ((0, 0), Direction::E, (10, 1));

    inst.into_iter().for_each(|(ins, v)| match ins {
        'F' => p = (p.0 + v * wp.0, p.1 + v * wp.1),
        'L' => {
            for _ in 0..v / 90 {
                let temp = wp.1;
                wp.1 = wp.0;
                wp.0 = -temp;
            }
        }
        'R' => {
            for _ in 0..v / 90 {
                let temp = wp.0;
                wp.0 = wp.1;
                wp.1 = -temp;
            }
        }
        'N' => wp = move_(&Direction::N, wp, v),
        'E' => wp = move_(&Direction::E, wp, v),
        'W' => wp = move_(&Direction::W, wp, v),
        'S' => wp = move_(&Direction::S, wp, v),
        _ => {}
    });

    (p.0.abs() + p.1.abs()) as usize
}

fn move_(d: &Direction, p: Position, v: isize) -> Position {
    match d {
        Direction::E => (p.0 + v, p.1),
        Direction::W => (p.0 - v, p.1),
        Direction::N => (p.0, p.1 + v),
        Direction::S => (p.0, p.1 - v),
    }
}

fn prepare_data(data: Vec<String>) -> Vec<(char, isize)> {
    data.into_iter()
        .map(|l| l.trim().to_string())
        .map(|l| {
            (
                l[0..1].parse::<char>().unwrap(),
                l[1..].parse::<isize>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

    #[test]
    fn should_solve() {
        let data = vec!["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .map(|e| e.to_string())
            .collect();

        assert_eq!(25, solve(prepare_data(data)));
    }

    #[test]
    fn should_solve_part_1() {
        assert_eq!(
            508,
            solve(prepare_data(parse_lines("input/day_12_data.txt")))
        );
    }

    #[test]
    fn should_solve_2() {
        let data = vec!["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .map(|e| e.to_string())
            .collect();

        assert_eq!(25, solve_2(prepare_data(data)));
    }

    #[test]
    fn should_solve_part_2() {
        assert_eq!(
            508,
            solve_2(prepare_data(parse_lines("input/day_12_data.txt")))
        );
    }
}
