type Map = Vec<Vec<char>>;

fn count_trees(map: Map) -> usize {
    count_trees_part_2(&map, 3, 1)
}

fn count_all(map: Map) -> usize {
    vec![
        count_trees_part_2(&map, 1, 1),
        count_trees_part_2(&map, 3, 1),
        count_trees_part_2(&map, 5, 1),
        count_trees_part_2(&map, 7, 1),
        count_trees_part_2(&map, 1, 2),
    ]
    .iter()
    .product()
}

#[allow(clippy::ptr_arg)]
fn count_trees_part_2(map: &Map, x_step: usize, y_step: usize) -> usize {
    let (mut count, mut x, mut y) = (0, 0, 0);
    loop {
        x = next_x(x, map[0].len(), x_step);
        y += y_step;

        if y >= map.len() {
            break;
        }

        if map[y][x] == '#' {
            count += 1;
        }
    }

    count
}

fn next_x(x: usize, len: usize, step: usize) -> usize {
    (x + step) % len
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
        assert_eq!(176, count_trees(map))
    }

    #[test]
    fn should_count_trees_day_3_input_part_2() {
        let map = create_map(parse_lines("input/day_3_data.txt"));
        assert_eq!(5872458240, count_all(map))
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

    #[test]
    fn should_count_trees_part_2() {
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
        assert_eq!(336, count_all(map))
    }
}
