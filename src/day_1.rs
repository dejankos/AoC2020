use itertools::Itertools;

const YEAR: usize = 2020;

fn find_sum(expenses: &[usize], k: usize) -> usize {
    expenses
        .iter()
        .copied()
        .combinations(k)
        .find(|v| v.iter().sum::<usize>() == YEAR)
        .expect("There should be a pair")
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use crate::data_parser;

    use super::*;

    #[test]
    fn should_find_sum_part_1() {
        assert_eq!(
            471019,
            find_sum(&data_parser::parse_file("input/day_1_data.txt"), 2)
        );
    }

    #[test]
    fn should_find_sum_part_2() {
        assert_eq!(
            103927824,
            find_sum(&data_parser::parse_file("input/day_1_data.txt"), 3)
        );
    }
}
