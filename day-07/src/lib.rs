use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, newline},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let map = lines
        .into_iter()
        .map(|(parent, children)| {
            (
                parent,
                children
                    .into_iter()
                    .filter(|&(count, _name)| count != 0)
                    .collect(),
            )
        })
        .collect::<HashMap<&str, Vec<(u32, &str)>>>();
    let mut visited = HashSet::new();
    let mut contain_gold = HashSet::new();
    let mut queue = VecDeque::new();
    for (&bag, children) in map.iter() {
        visited.clear();
        visited.insert(bag);
        queue.clear();
        queue.extend(children.iter().filter_map(|&(_, name)| {
            if !visited.contains(name) {
                Some(name)
            } else {
                None
            }
        }));
        while !queue.is_empty() {
            let child = queue.pop_front().unwrap();
            visited.insert(child);
            if child == "shiny gold" {
                contain_gold.insert(bag);
            }
            queue.extend(map.get(child).unwrap().iter().filter_map(|&(_, name)| {
                if !visited.contains(name) {
                    Some(name)
                } else {
                    None
                }
            }));
        }
    }
    contain_gold.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let map = lines
        .into_iter()
        .map(|(parent, children)| {
            (
                parent,
                children
                    .into_iter()
                    .filter(|&(count, _name)| count != 0)
                    .collect(),
            )
        })
        .collect::<HashMap<&str, Vec<(u32, &str)>>>();
    let total = count_bags("shiny gold", &map) - 1;
    total.to_string()
}

fn count_bags(bag: &str, map: &HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    let children = map.get(bag).unwrap();
    if children.is_empty() {
        1
    } else {
        children.iter().fold(1, |acc, el| {
            let &(count, child) = el;
            acc + count * count_bags(child, map)
        })
    }
}

type Line<'a> = (&'a str, Vec<(u32, &'a str)>);

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, parent) = parse_bag_name(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, children) = separated_list1(tag(", "), parse_child_bag)(input)?;
    let (input, _) = tag(".")(input)?;
    Ok((input, (parent, children)))
}

fn parse_child_bag(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, count) = parse_bag_count(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, bag) = parse_bag_name(input)?;
    Ok((input, (count, bag)))
}

fn parse_bag_count(input: &str) -> IResult<&str, u32> {
    let (input, count) = take_until(" ")(input)?;
    let num = match count {
        "no" => 0,
        _ => parse_u32(count).unwrap().1,
    };
    Ok((input, num))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, number) = map_res(recognize(digit1), |s: &str| s.parse::<u32>())(input)?;
    Ok((input, number))
}

fn parse_bag_name(input: &str) -> IResult<&str, &str> {
    let (input, bag_name) = take_until(" bag")(input)?;
    let (input, _) = parse_bag_tag(input)?;
    Ok((input, bag_name))
}

fn parse_bag_tag(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = many0(tag("s"))(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let result = process_part1(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn part2() {
        let input1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let result1 = process_part2(input1);
        assert_eq!(result1, "32");

        let input2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let result2 = process_part2(input2);
        assert_eq!(result2, "126");
    }
}
