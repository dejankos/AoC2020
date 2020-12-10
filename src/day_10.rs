fn solve(mut input: Vec<usize>) -> usize {
    input.sort_unstable();

    let (un, tres, _) = input
        .into_iter()
        .fold((0, 1, 0), |(o, t, last), n| match n - last {
            1 => (o + 1, t, n),
            3 => (o, t + 1, n),
            _ => (o, t, n),
        });
    un * tres
}

fn solve_part_2(mut input: Vec<usize>) -> usize {
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort_unstable();

    let mut m = vec![0; input.len()];
    m[0] = 1;

    for i in 1..input.len() {
        if input[i] - input[i - 1] <= 3 {
            m[i] += m[i - 1];
        }

        if i > 1 && input[i] - input[i - 2] <= 3 {
            m[i] += m[i - 2];
        }

        if i > 2 && input[i] - input[i - 3] <= 3 {
            m[i] += m[i - 3];
        }
    }

    *m.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_file;

    use super::*;

    #[test]
    fn should_solve() {
        let data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(35, solve(data));
    }

    #[test]
    fn should_solve_example_2() {
        let data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(220, solve(data));
    }

    #[test]
    fn should_solve_part_1() {
        assert_eq!(2592, solve(parse_file("input/day_10_data.txt")));
    }

    #[test]
    fn should_solve_example_part_2() {
        let data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(8, solve_part_2(data));
    }

    #[test]
    fn should_solve_part_2() {
        assert_eq!(198428693313536, solve_part_2(parse_file("input/day_10_data.txt")));
    }
}
