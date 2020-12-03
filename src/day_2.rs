type Passwords = Vec<(usize, usize, char, String)>;

fn find_valid(pass: Passwords) -> usize {
    pass.into_iter().fold(0, |acc, (min, max, c, password)| {
        let n = n_occurrences(password, c);
        if n >= min && n <= max {
            acc + 1
        } else {
            acc
        }
    })
}

fn n_occurrences(password: String, c: char) -> usize {
    password.chars().fold(0, |acc, ch| acc + (c == ch) as usize)
}

fn find_valid_part_2(pass: Passwords) -> usize {
    pass.into_iter().fold(0, |acc, (min, max, c, password)| {
        let (f, l) = (
            password.chars().nth(min - 1).unwrap(),
            password.chars().nth(max - 1).unwrap(),
        );

        acc + match (f == c, l == c) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 1,
            _ => 0,
        }
    })
}

fn prepare_data(data: Vec<String>) -> Passwords {
    data.into_iter()
        .map(|line| {
            let split = line
                .split_whitespace()
                .flat_map(|s| s.split('-'))
                .flat_map(|s| s.split(':'))
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            (
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].chars().next().unwrap(),
                split[3].into(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_parser::parse_lines;

    #[test]
    fn should_find_valid_pass() {
        let data = prepare_data(vec![
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ]);

        assert_eq!(2, find_valid(data));
    }

    #[test]
    fn should_find_valid_pass_part_2() {
        let data = prepare_data(vec![
            "1-3 a: abcde".into(),
            "1-3 b: cdefg".into(),
            "2-9 c: ccccccccc".into(),
        ]);

        assert_eq!(1, find_valid_part_2(data));
    }

    #[test]
    fn should_find_valid_pass_day_2_data() {
        let data = prepare_data(parse_lines("input/day_2_data.txt"));
        assert_eq!(660, find_valid(data));
    }

    #[test]
    fn should_find_valid_pass_day_2_data_part_2() {
        let data = prepare_data(parse_lines("input/day_2_data.txt"));
        assert_eq!(530, find_valid_part_2(data));
    }

    #[test]
    fn should_print_prepared_data() {
        println!(
            "{:?}",
            prepare_data(vec![
                "1-3 a: abcde".into(),
                "1-3 b: cdefg".into(),
                "2-9 c: ccccccccc".into()
            ])
        )
    }
}
