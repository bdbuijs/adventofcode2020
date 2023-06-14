use nom::{
    bytes::complete::{tag, take_until},
    character::{
        complete::space1,
        complete::{alpha1, newline, one_of},
    },
    combinator::recognize,
    multi::{many0, many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

type Line<'a> = Vec<&'a str>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, parent) = parse_bag_name(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, children) = separated_list1(",", parse_bag_name)(input)?;
    dbg!(parent);
    dbg!(&children);

    // Ok((input, line))
    todo!()
}

fn parse_bag_name(input: &str) -> IResult<&str, &str> {
    let (input, bag_name) = take_until(" bag")(input)?;
    let (input, _) = parse_bag(input)?;
    Ok((input, bag_name))
}

fn parse_bag(input: &str) -> IResult<&str, ()> {
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
        assert_eq!(result, "");
    }

    #[test]
    fn part2() {
        let input = "";
        let result = process_part2(input);
        assert_eq!(result, "");
    }
}
