use itertools::Itertools;
use nom::{character::complete::newline, multi::separated_list1, IResult};

pub fn process_part1(input: &str) -> String {
    let (_, entries) = parse_input(input).unwrap();
    for comb in entries.into_iter().combinations(2) {
        let (a, b) = (comb[0], comb[1]);
        if a + b == 2020 {
            return (a * b).to_string();
        }
    }
    "Not found".to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, entries) = parse_input(input).unwrap();
    for comb in entries.into_iter().combinations(3) {
        let (a, b, c) = (comb[0], comb[1], comb[2]);
        if a + b + c == 2020 {
            return (a * b * c).to_string();
        }
    }
    "Not found".to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, line) = nom::character::complete::u32(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1721
979
366
299
675
1456";
        let result = process_part1(input);
        assert_eq!(result, "514579");
    }

    #[test]
    fn part2() {
        let input = "1721
979
366
299
675
1456";
        let result = process_part2(input);
        assert_eq!(result, "241861950");
    }
}
