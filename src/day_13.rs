use std::collections::HashMap;

fn solve(input: (usize, Vec<usize>)) -> usize {
    let ts = input.0;

    let min = input
        .1
        .into_iter()
        .map(|id| (find_first(id, ts), id))
        .min_by_key(|p| p.0)
        .unwrap();

    (min.0 - ts) * min.1
}

fn find_first(id: usize, ts: usize) -> usize {
    let mut i = ts;

    while i % id != 0 {
        i += 1;
    }

    i
}

fn prepare_data(data: Vec<String>) -> (usize, Vec<usize>) {
    let ts = data[0].parse::<usize>().unwrap();
    let ids = data[1]
        .split(',')
        .into_iter()
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (ts, ids)
}

fn prepare_data_part_2(data: Vec<String>) -> Vec<(usize, usize)> {
    data[1]
        .split(',')
        .into_iter()
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (s.parse::<usize>().unwrap(), i))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

    #[test]
    fn should_solve() {
        let data = vec!["939".into(), "7,13,x,x,59,x,31,19".into()];

        assert_eq!(295, solve(prepare_data(data)));
    }

    #[test]
    fn should_solve_part_1() {
        assert_eq!(
            2845,
            solve(prepare_data(parse_lines("input/day_13_data.txt")))
        );
    }
}
