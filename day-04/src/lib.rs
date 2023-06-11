use std::collections::HashMap;

use nom::character::{
    complete::space1,
    complete::{alpha1, newline, one_of},
};
use nom::{
    bytes::complete::tag,
    combinator::recognize,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let input = input.replace("\n\n", "$$$$$");
    let input = input.replace("\n", " ");
    let input = input.replace("$$$$$", "\n");
    let (_, passports) = parse_input(&input).unwrap();
    let count = passports
        .into_iter()
        .filter_map(|passport| {
            let map: HashMap<&str, &str> = passport.into_iter().collect();
            if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .into_iter()
                .all(|k| map.contains_key(k))
            {
                Some(1)
            } else {
                None
            }
        })
        .sum::<u32>();

    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let input = input.replace("\n\n", "$$$$$");
    let input = input.replace("\n", " ");
    let input = input.replace("$$$$$", "\n");
    let (_, passports) = parse_input(&input).unwrap();
    let count = passports
        .into_iter()
        .filter_map(|passport| {
            let map: HashMap<&str, &str> = passport.into_iter().collect();
            if !(map.contains_key(&"byr")
                && (1920..=2002).contains(&map.get(&"byr").unwrap().parse::<u32>().unwrap()))
                || !(map.contains_key(&"iyr")
                    && (2010..=2020).contains(&map.get(&"iyr").unwrap().parse::<u32>().unwrap()))
                || !(map.contains_key(&"eyr")
                    && (2020..=2030).contains(&map.get(&"eyr").unwrap().parse::<u32>().unwrap()))
                || !(map.contains_key(&"hgt") && {
                    let &s = map.get(&"hgt").unwrap();
                    let unit = &s[(s.len() - 2)..];
                    let range = match unit {
                        "in" => 59..=76,
                        "cm" => 150..=193,
                        _ => return None,
                    };
                    let value = s[..s.len() - 2].parse::<u32>().unwrap();
                    range.contains(&value)
                })
                || !(map.contains_key(&"hcl") && {
                    let &s = map.get(&"hcl").unwrap();
                    s.len() == 7
                        && s[1..]
                            .chars()
                            .all(|c| nom::character::is_hex_digit(c as u8))
                })
                || !(map.contains_key(&"ecl") && {
                    let &s = map.get(&"ecl").unwrap();
                    match s {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    }
                })
                || !(map.contains_key(&"pid") && {
                    let &s = map.get(&"pid").unwrap();
                    s.len() == 9 && s.chars().all(|c| nom::character::is_digit(c as u8))
                })
            {
                None
            } else {
                Some(1)
            }
        })
        .sum::<u32>();
    count.to_string()
}

type Line<'a> = Vec<(&'a str, &'a str)>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(space1, parse_pair)(input)?;
    Ok((input, line))
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, key) = alpha1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = recognize(many1(one_of(
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ#",
    )))(input)?;
    Ok((input, (key, value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let invalid_count = process_part2(invalid);
        let valid_count = process_part2(valid);
        assert_eq!(invalid_count, "0");
        assert_eq!(valid_count, "4");
    }
}
