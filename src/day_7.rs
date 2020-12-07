use std::collections::HashMap;

use petgraph::stable_graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;

#[derive(Debug, Eq, PartialEq)]
struct Color {
    n: usize,
    name: String,
}

fn solve(data: Vec<Vec<Color>>, node_name: &str) -> usize {
    let mut graph = Graph::<&str, usize>::new();
    let mut map: HashMap<&str, NodeIndex<u32>> = HashMap::new();

    data.iter().for_each(|n| {
        let first = &n[0];
        n.iter().for_each(|c| {
            if !map.contains_key(c.name.as_str()) {
                let n_idx = graph.add_node(&c.name);
                map.insert(&c.name, n_idx);
            }
            if first != c {
                graph.add_edge(map[first.name.as_str()], map[c.name.as_str()], c.n);
            }
        })
    });

    let (search, mut count) = (map[node_name], 0);
    for start in graph.node_indices() {
        let mut dfs = Dfs::new(&graph, start);

        if start.index() != search.index() {
            while let Some(visited) = dfs.next(&graph) {
                if visited.index() == search.index() {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_part_2(data: Vec<Vec<Color>>, node_name: &str) -> usize {
    let mut map: HashMap<String, Vec<Color>> = HashMap::new();

    for mut v in data.into_iter() {
        let f = v.remove(0);
        map.insert(f.name, v);
    }

    let mut r = 0;
    for i in map[node_name].iter() {
        r += i.n + (i.n * find_all(i.name.as_str(), &map))
    }

    r
}

fn find_all(name: &str, map: &HashMap<String, Vec<Color>>) -> usize {
    if !map.contains_key(name) {
        0
    } else {
        let v = map.get(name).unwrap();
        let mut n = 0;
        for c in v.iter() {
            n += c.n + c.n * find_all(c.name.as_str(), map);
        }
        n
    }
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
        assert_eq!(205, solve(data, "shinygold"));
    }

    #[test]
    fn should_solve_part_2() {
        let p = prepare_data(parse_lines("input/day_7_data.txt"));
        assert_eq!(80902, solve_part_2(p, "shinygold".into()));
    }
}
