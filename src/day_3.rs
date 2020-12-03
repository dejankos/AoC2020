type Map = Vec<Vec<char>>;

fn count_trees(map: Map) -> usize {
    let (mut count, mut x, mut y) = (0, 0, 0);

    loop {
        x = next_x(x, map[0].len());
        y += 1;

        if y == map.len() {
            break;
        }

        if map[y][x] == '#' {
            count += 1;
        }
    }

    count
}

fn next_x(x: usize, len: usize) -> usize {
    (x + 3) % len
}

fn create_map(input: Vec<String>) -> Map {
    input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

    #[test]
    fn should_count_trees_day_3_input() {
        let map = create_map(parse_lines("input/day_3_data.txt"));
        assert_eq!(7, count_trees(map))
    }

    #[test]
    fn should_count_trees() {
        let data = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        let map = create_map(data.lines().map(|l| l.into()).collect());
        assert_eq!(7, count_trees(map))
    }
}
