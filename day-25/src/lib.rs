use nom::{
    character::complete::digit1, character::complete::newline, multi::separated_list1, IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, keys) = parse_input(input).unwrap();
    assert_eq!(keys.len(), 2);
    let card = keys[0];
    let door = keys[1];
    let mut n = 1;
    let mut card_loop = 0;
    while n != card {
        n = do_loop(n, 7);
        card_loop += 1;
    }
    n = 1;
    (0..card_loop).for_each(|_| {
        n = do_loop(n, door);
    });

    n.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[inline(always)]
fn do_loop(n: usize, subject_number: usize) -> usize {
    (n * subject_number) % 20201227
}

type Line = usize;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = digit1(input)?;
    Ok((input, line.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5764801
17807724";
        let result = process_part1(input);
        assert_eq!(result, "14897079");
    }

    #[test]
    fn part2() {
        let input = "";
        let result = process_part2(input);
        assert_eq!(result, "");
    }
}
