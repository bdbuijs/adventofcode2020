use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_until1},
    multi::{many0, many1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, groups) = parse_input(input).unwrap();
    let sum: usize = groups
        .into_iter()
        .map(|s| {
            let set: HashSet<char> = s.into_iter().flat_map(|s| s.chars()).collect();
            set.len()
        })
        .sum();
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, groups) = parse_input(input).unwrap();
    groups
        .into_iter()
        .map(|group| {
            let all: HashSet<char> = group[0].chars().collect();
            group
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .fold(all, |acc, el| {
                    acc.intersection(&el).into_iter().map(|&e| e).collect()
                })
        })
        .map(|set| set.len())
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, groups) = many0(block)(input)?;
    Ok((input, groups))
}

fn block(i: &str) -> IResult<&str, Vec<&str>> {
    let (i, idents) = many1(nonempty)(i)?;
    let (i, _) = many0(tag("\n"))(i)?;
    Ok((i, idents))
}

fn nonempty(i: &str) -> IResult<&str, &str> {
    let (i, ident) = take_until1("\n")(i)?;
    let (i, _) = tag("\n")(i)?;
    Ok((i, ident))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
        let result = process_part1(input);
        assert_eq!(result, "11");
    }

    #[test]
    fn part2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
        let result = process_part2(input);
        assert_eq!(result, "6");
    }
}
