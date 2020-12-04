use std::path::Path;

pub fn parse_file<P>(path: P) -> Vec<usize>
where
    P: AsRef<Path>,
{
    parse(path, |s| {
        s.parse().expect("Not the parse you're looking for")
    })
}

pub fn parse_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    parse(path, |s| s.into())
}

pub fn parse_on_blank_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let split_reg = regex::Regex::new("\n\\s*\n").unwrap();
    split_reg
        .split(&load_file(path))
        .map(|l| l.replace("\n", " "))
        .collect()
}

fn parse<T, P, F>(path: P, f: F) -> Vec<T>
where
    P: AsRef<Path>,
    F: Fn(&str) -> T,
{
    load_file(path).lines().map(|s| f(s)).collect()
}

fn load_file<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(path).expect("Can't load file")
}
