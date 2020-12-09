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

fn find_contiguous_set(input: Vec<usize>, sum: usize) -> usize {
    let mut res = 0;
    input.iter().copied().enumerate().any(|(i, n)| {
        let mut f = find_subset(&input[i + 1..input.len()], sum, n);
        if !f.is_empty() {
            f.push(n);
            res = f.iter().max().unwrap() + f.iter().min().unwrap();
            true
        } else {
            false
        }
    });

    res
}

fn find_subset(set: &[usize], sum: usize, start: usize) -> Vec<usize> {
    let mut s = start;
    for (i, n) in set.into_iter().enumerate() {
        s += n;
        if s == sum {
            return set[0..=i].to_vec();
        } else if s > sum {
            return vec![];
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_file;

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

    #[test]
    fn should_solve_part_2() {
        let inpuit = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        println!("{}", find_contiguous_set(inpuit, 127));
    }

    #[test]
    fn should_solve_day_1_part_2() {
        let data = parse_file("input/day_9_data.txt");
        assert_eq!(1766397, find_contiguous_set(data.clone(), solve(data, 25)));
    }
}
