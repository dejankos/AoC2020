use std::collections::HashMap;

#[derive(Debug)]
struct Color {
    n: usize,
    name: String,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn solve_part_1(data: Vec<Vec<Color>>, find: &Color) -> usize {
    let map = convert(data);
    map.iter()
        .map(|(n, _)| find_all_paths(n, find, &map) as usize)
        .sum()
}

fn find_all_paths(from: &str, to: &Color, map: &HashMap<String, Vec<Color>>) -> bool {
    if !map.contains_key(from) {
        false
    } else {
        let v = map.get(from).unwrap();
        if v.contains(to) {
            true
        } else {
            v.iter()
                .map(|c| find_all_paths(c.name.as_str(), to, map))
                .any(|b| b == true)
        }
    }
}

fn solve_part_2(data: Vec<Vec<Color>>, node_name: &str) -> usize {
    let map = convert(data);
    find_all(node_name, &map)
}

fn find_all(name: &str, map: &HashMap<String, Vec<Color>>) -> usize {
    if !map.contains_key(name) {
        0
    } else {
        map.get(name)
            .unwrap()
            .iter()
            .fold(0, |acc, c| acc + c.n + c.n * find_all(c.name.as_str(), map))
    }
}

fn convert(data: Vec<Vec<Color>>) -> HashMap<String, Vec<Color>> {
    let mut map: HashMap<String, Vec<Color>> = HashMap::new();
    for mut v in data.into_iter() {
        let f = v.remove(0);
        map.insert(f.name, v);
    }

    map
}

fn prepare_data(data: Vec<String>) -> Vec<Vec<Color>> {
    data.into_iter()
        .filter(|l| !l.contains("contain no"))
        .map(parse_line)
        .collect()
}

fn parse_line(l: String) -> Vec<Color> {
    let clear = regex::Regex::new("bags|contain|bag|[,]|[.]").unwrap();
    clear
        .split(&l)
        .map(|s| s.trim().to_string())
        .fold(Vec::new(), |mut acc, n| {
            if n.is_empty() {
                acc
            } else {
                let s = n
                    .split_whitespace()
                    .map(|s| s.into())
                    .collect::<Vec<String>>();
                if let Ok(v) = s[0].parse::<usize>() {
                    acc.push(Color {
                        n: v,
                        name: s.into_iter().skip(1).collect::<String>(),
                    });
                    acc
                } else {
                    acc.push(Color {
                        n: 0,
                        name: s.into_iter().collect::<String>(),
                    });
                    acc
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_lines;

    use super::*;

    #[test]
    fn should_solve() {
        let data = prepare_data(parse_lines("input/day_7_data.txt"));
        assert_eq!(
            205,
            solve_part_1(
                data,
                &Color {
                    n: 0,
                    name: "shinygold".into()
                }
            )
        );
    }

    #[test]
    fn should_solve_part_2() {
        let p = prepare_data(parse_lines("input/day_7_data.txt"));
        assert_eq!(80902, solve_part_2(p, "shinygold".into()));
    }
}
