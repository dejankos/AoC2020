use std::path::Path;

pub fn parse_file<P>(path: P) -> Vec<usize>
where
    P: AsRef<Path>,
{
    load_file(path)
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn load_file<P>(path: P) -> String
where
    P: AsRef<Path>
{
    std::fs::read_to_string(path).expect("Can't load file")
}
