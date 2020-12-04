use std::num::ParseIntError;

type Pair = (String, String);
type PairBatch = Vec<Vec<Pair>>;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new(pairs: Vec<Pair>) -> Self {
        let mut pass = Passport::default();
        pairs.into_iter().for_each(|(k, v)| match k.as_str() {
            "byr" => pass.byr = Some(v),
            "iyr" => pass.iyr = Some(v),
            "eyr" => pass.eyr = Some(v),
            "hgt" => pass.hgt = Some(v),
            "hcl" => pass.hcl = Some(v),
            "ecl" => pass.ecl = Some(v),
            "pid" => pass.pid = Some(v),
            "cid" => pass.cid = Some(v),
            _ => panic!("unknown property"),
        });

        pass
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part_2(&self) -> bool {
        self.is_valid()
            && valid_number(self.byr.as_ref().unwrap(), 1920, 2002).unwrap_or(false)
            && valid_number(self.iyr.as_ref().unwrap(), 2010, 2020).unwrap_or(false)
            && valid_number(self.eyr.as_ref().unwrap(), 2020, 2030).unwrap_or(false)
            && valid_height(self.hgt.as_ref().unwrap()).unwrap_or(false)
            && valid_hair_clr(self.hcl.as_ref().unwrap())
            && valid_eye_clr(self.ecl.as_ref().unwrap())
            && valid_pid(self.pid.as_ref().unwrap()).unwrap_or(false)
    }
}

fn valid_hair_clr(n: &str) -> bool {
    let prefix = &n[0..1];
    let sufix = &n[1..];

    prefix == "#" && sufix.len() == 6 && sufix.chars().all(|x| x.is_alphanumeric())
}

fn valid_eye_clr(n: &str) -> bool {
    matches!(n, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn valid_number(n: &str, min: usize, max: usize) -> Result<bool, ParseIntError> {
    let num = n.parse::<usize>()?;
    if num >= min && num <= max {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn valid_height(n: &str) -> Result<bool, ParseIntError> {
    let unit = &n[n.len() - 2..];
    let h = &n[..n.len() - 2].parse::<usize>()?;
    match unit {
        "cm" => Ok(*h >= 150 && *h <= 193),
        "in" => Ok(*h >= 59 && *h <= 76),
        _ => Ok(false),
    }
}

fn valid_pid(n: &str) -> Result<bool, ParseIntError> {
    let _num = n.parse::<usize>()?;
    Ok(n.len() == 9)
}

fn count_valid_part_1(batch: PairBatch) -> usize {
    count_valid(batch, |p| p.is_valid())
}

fn count_valid_part_2(batch: PairBatch) -> usize {
    count_valid(batch, |p| p.is_valid_part_2())
}

fn count_valid<F: Fn(Passport) -> bool>(batch: PairBatch, f: F) -> usize {
    batch
        .into_iter()
        .map(Passport::new)
        .map(|p| f(p) as usize)
        .filter(|v| *v == 1)
        .count()
}

fn parse_data(input: Vec<String>) -> PairBatch {
    input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .map(|p| p.into())
                .collect::<Vec<String>>()
                .into_iter()
                .map(|pair| {
                    let mut iter = pair.split(':');
                    (iter.next().unwrap().into(), iter.next().unwrap().into())
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_on_blank_lines;

    use super::*;

    #[test]
    fn should_count_valid_passwords() {
        let raw = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let reg = regex::Regex::new("\n\\s*\n").unwrap();
        let data: Vec<String> = reg.split(raw).map(|l| l.replace("\n", " ")).collect();

        assert_eq!(2, count_valid_part_1(parse_data(data)));
    }

    #[test]
    fn should_count_valid_passwords_day_4_input() {
        let data = parse_data(parse_on_blank_lines("input/day_4_data.txt"));
        assert_eq!(260, count_valid_part_1(data));
    }

    #[test]
    fn should_count_valid_passwords_part_2() {
        let raw = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

        let reg = regex::Regex::new("\n\\s*\n").unwrap();
        let data: Vec<String> = reg.split(raw).map(|l| l.replace("\n", " ")).collect();
        assert_eq!(4, count_valid_part_2(parse_data(data)));
    }

    #[test]
    fn should_count_invalid_passwords_part_2() {
        let raw = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

        let reg = regex::Regex::new("\n\\s*\n").unwrap();
        let data: Vec<String> = reg.split(raw).map(|l| l.replace("\n", " ")).collect();
        assert_eq!(0, count_valid_part_2(parse_data(data)));
    }

    #[test]
    fn should_count_valid_passwords_day_4_input_part_2() {
        let data = parse_data(parse_on_blank_lines("input/day_4_data.txt"));
        assert_eq!(153, count_valid_part_2(data));
    }
}
