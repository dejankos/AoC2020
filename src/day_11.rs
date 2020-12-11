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

fn solve_2(seats: Vec<Vec<char>>) -> usize {
    let mut out = solve_part_2(seats);
    loop {
        let res = solve_part_2(out.clone());
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

fn solve_part_2(mut seats: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let x_len = seats.len();
    let y_len = seats[0].len();
    let state = seats.clone();

    for x in 0..x_len {
        for y in 0..y_len {
            let c = occupied_count_part_2(&state, x as isize, y as isize);
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
                    if c >= 5 {
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

fn occupied_count_part_2(seats: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    look_in_direction(seats, |x, y| (x - 1, y), x, y)
        + look_in_direction(seats, |x, y| (x + 1, y), x, y)
        + look_in_direction(seats, |x, y| (x, y - 1), x, y)
        + look_in_direction(seats, |x, y| (x, y + 1), x, y)
        + look_in_direction(seats, |x, y| (x + 1, y + 1), x, y)
        + look_in_direction(seats, |x, y| (x + 1, y - 1), x, y)
        + look_in_direction(seats, |x, y| (x - 1, y + 1), x, y)
        + look_in_direction(seats, |x, y| (x - 1, y - 1), x, y)
}

fn look_in_direction<F: Fn(isize, isize) -> (isize, isize)>(
    seats: &Vec<Vec<char>>,
    f: F,
    x: isize,
    y: isize,
) -> usize {
    let (mut r, mut x, mut y) = (0, x, y);
    loop {
        let (i, j) = f(x, y);
        x = i;
        y = j;
        let (c, exists, empty) = occupied_2(seats, x, y);
        if !exists || empty {
            break;
        }
        if c > 0 {
            r = c;
            break;
        }
    }

    r
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

fn occupied_2(seats: &Vec<Vec<char>>, x: isize, y: isize) -> (usize, bool, bool) {
    if (x >= 0 && y >= 0)
        && (x <= seats.len() as isize - 1 && y <= seats[x as usize].len() as isize - 1)
    {
        (
            (seats[x as usize][y as usize] == '#') as usize,
            true,
            seats[x as usize][y as usize] == 'L',
        )
    } else {
        (0, false, false)
    }
}

fn prepare_data(data: Vec<String>) -> Vec<Vec<char>> {
    data.into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

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
    fn should_solve_2() {
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

        assert_eq!(26, solve_2(prepare_data(data)));
    }

    #[test]
    fn should_solve_part_1() {
        assert_eq!(
            2270,
            solve_part_1(prepare_data(parse_lines("input/day_11_data.txt")))
        );
    }

    #[test]
    fn should_solve_part_2() {
        assert_eq!(
            2042,
            solve_2(prepare_data(parse_lines("input/day_11_data.txt")))
        );
    }
}
