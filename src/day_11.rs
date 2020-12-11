fn solve_part_1(seats: Vec<Vec<char>>) -> usize {
    let mut out = solve(seats);
    loop {
        let res = solve(out.clone());
        let matching = res.iter().zip(out.iter()).filter(|&(a, b)| a == b).count();
        if matching == res.len() {
            break;
        }

        out = res;
    }

    out.iter().flatten().filter(|c| **c == '#').count()
}

fn solve(mut seats: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let x_len = seats.len();
    let y_len = seats[0].len();
    let state = seats.clone();

    for x in 0..x_len {
        for y in 0..y_len {
            let c = occupied_count(&state, x as isize, y as isize);
            match &seats[x][y] {
                '.' => {}
                'L' => {
                    if c == 0 {
                        seats[x][y] = '#';
                    } else {
                        seats[x][y] = 'L';
                    }
                }
                '#' => {
                    if c >= 4 {
                        seats[x][y] = 'L';
                    } else {
                        seats[x][y] = '#';
                    }
                }
                _ => panic!("unknown state"),
            }
        }
    }

    seats.clone()
}

fn occupied_count(seats: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    occupied(seats, x - 1, y)
        + occupied(seats, x + 1, y)
        + occupied(seats, x, y - 1)
        + occupied(seats, x, y + 1)
        + occupied(seats, x + 1, y + 1)
        + occupied(seats, x + 1, y - 1)
        + occupied(seats, x - 1, y + 1)
        + occupied(seats, x - 1, y - 1)
}

fn occupied(seats: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    if (x >= 0 && y >= 0)
        && (x <= seats.len() as isize - 1 && y <= seats[x as usize].len() as isize - 1)
    {
        (seats[x as usize][y as usize] == '#') as usize
    } else {
        0
    }
}

fn prepare_data(data: Vec<String>) -> Vec<Vec<char>> {
    data.into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_parser::parse_lines;

    #[test]
    fn should_solve() {
        let data = vec![
            "L.LL.LL.LL".into(),
            "LLLLLLL.LL".into(),
            "L.L.L..L..".into(),
            "LLLL.LL.LL".into(),
            "L.LL.LL.LL".into(),
            "L.LLLLL.LL".into(),
            "..L.L.....".into(),
            "LLLLLLLLLL".into(),
            "L.LLLLLL.L".into(),
            "L.LLLLL.LL".into(),
        ];

        assert_eq!(37, solve_part_1(prepare_data(data)));
    }

    #[test]
    fn should_solve_part_1() {
        assert_eq!(
            2270,
            solve_part_1(prepare_data(parse_lines("input/day_11_data.txt")))
        );
    }
}
