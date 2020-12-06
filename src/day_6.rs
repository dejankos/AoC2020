use std::collections::HashSet;

fn sum_all_groups(a: Vec<HashSet<char>>) -> usize {
    a.into_iter().map(|h| h.len()).sum()
}

fn parse_data(data: Vec<String>) -> Vec<HashSet<char>> {
    data.into_iter()
        .map(|l| l.split_whitespace().collect::<String>())
        .map(|l| l.chars().into_iter().collect::<HashSet<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_on_blank_lines;

    use super::*;

    #[test]
    fn should_find_sum_of_all_groups() {
        assert_eq!(
            6530,
            sum_all_groups(parse_data(parse_on_blank_lines("input/day_6_data.txt")))
        );
    }
}
