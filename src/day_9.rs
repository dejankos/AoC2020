use itertools::Itertools;

fn solve(input: Vec<usize>, preamble: usize) -> usize {
    input
        .iter()
        .copied()
        .enumerate()
        .skip(preamble)
        .find(|(i, n)| !is_valid(&input[i - (preamble)..=i - 1], *n))
        .unwrap()
        .1
}

fn is_valid(slice: &[usize], n: usize) -> bool {
    slice
        .iter()
        .combinations(2)
        .any(|v| v.iter().copied().sum::<usize>() == n)
}

#[cfg(test)]
mod tests {
    use crate::data_parser::{parse_file};

    use super::*;

    #[test]
    fn should_solve() {
        let inpuit = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(127, solve(inpuit, 5));
    }

    #[test]
    fn should_solve_day_1_data() {
        assert_eq!(14144619, solve(parse_file("input/day_9_data.txt"), 25));
    }
}
