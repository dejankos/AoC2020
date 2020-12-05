use crate::data_parser::parse_lines;

fn find_highest_seat_id(data: Vec<(String, String)>) -> usize {
    data.into_iter()
        .map(|mut tup| bsp(&mut tup.0, 0, 127) * 8 + bsp(&mut tup.1, 0, 7))
        .max()
        .expect("should find something")
}

fn find_my_seat(data: Vec<(String, String)>) -> usize {
    let mut seats = data
        .into_iter()
        .map(|mut tup| bsp(&mut tup.0, 0, 127) * 8 + bsp(&mut tup.1, 0, 7))
        .collect::<Vec<usize>>();
    seats.sort_unstable();

    (seats[0]..=seats[seats.len() - 1])
        .into_iter()
        .sum::<usize>()
        - seats.iter().sum::<usize>()
}

fn bsp(cmd: &mut String, low: usize, up: usize) -> usize {
    if let Some(c) = cmd.pop() {
        match c {
            'F' | 'L' => {
                if cmd.is_empty() {
                    low
                } else {
                    bsp(cmd, low, low + ((up - low) / 2))
                }
            }
            'B' | 'R' => {
                if cmd.is_empty() {
                    up
                } else {
                    bsp(cmd, 1 + low + ((up - low) / 2), up)
                }
            }
            _ => panic!("unknown cmd"),
        }
    } else {
        panic!("should not happen")
    }
}

fn prepare_data() -> Vec<(String, String)> {
    parse_lines("input/day_5_data.txt")
        .into_iter()
        .map(|line| line.trim().to_string())
        .map(|line| {
            (
                line[0..7].chars().rev().collect(),
                line[7..].chars().rev().collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_bsp() {
        let mut s = "FBFBBFF".chars().rev().collect::<String>();
        assert_eq!(44, bsp(&mut s, 0, 127))
    }

    #[test]
    fn should_calc_bsp_2() {
        let mut s = "RLR".chars().rev().collect::<String>();
        assert_eq!(5, bsp(&mut s, 0, 7))
    }

    #[test]
    fn should_find_highest_seat_id_part_1() {
        assert_eq!(980, find_highest_seat_id(prepare_data()));
    }

    #[test]
    fn should_find_my_seat() {
        assert_eq!(607, find_my_seat(prepare_data()));
    }
}
