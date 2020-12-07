use std::collections::HashSet;

type Answers = Vec<HashSet<char>>;

fn sum_all_groups(a: Answers) -> usize {
    a.into_iter().map(|h| h.len()).sum()
}

fn sum_all_unique(a: Vec<Answers>) -> usize {
    a.into_iter().map(unique_count).sum()
}

fn unique_count(mut a: Answers) -> usize {
    let last = a.pop().unwrap();
    a.into_iter()
        .fold(last, |acc, n| acc.intersection(&n).copied().collect())
        .len()
}

fn parse_data(data: Vec<String>) -> Answers {
    data.into_iter()
        .map(|l| l.split_whitespace().collect::<String>())
        .map(|l| l.chars().into_iter().collect())
        .collect()
}

fn parse_data_part_2(data: Vec<String>) -> Vec<Answers> {
    data.into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.into())
                .collect::<Vec<String>>()
        })
        .map(|a| {
            a.into_iter()
                .map(|s| s.chars().into_iter().collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_parser::parse_on_blank_lines;

    #[test]
    fn should_find_sum_of_all_groups() {
        assert_eq!(
            6530,
            sum_all_groups(parse_data(parse_on_blank_lines("input/day_6_data.txt")))
        );
    }

    #[test]
    fn should_count_all_unique() {
        assert_eq!(3, unique_count(vec![set!['a', 'b', 'c']]));
        assert_eq!(1, unique_count(vec![set!['a', 'b'], set!['a', 'c']]));
    }

    #[test]
    fn should_find_sum_of_all_unique_answers() {
        assert_eq!(
            3323,
            sum_all_unique(parse_data_part_2(parse_on_blank_lines(
                "input/day_6_data.txt"
            )))
        );
    }
}
