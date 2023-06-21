use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult};

pub fn process_part1(input: &str) -> String {
    let (_, mut numbers) = parse_line(input).unwrap();
    while numbers.len() < 2020 {
        let last_number = *numbers.last().unwrap();
        if let Some(new_number) = numbers.iter().rev().skip(1).position(|&n| n == last_number) {
            numbers.push(new_number + 1);
        } else {
            numbers.push(0);
        }
    }
    numbers.last().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, numbers) = parse_line(input).unwrap();
    let len = numbers.len();
    let mut last = *numbers.last().unwrap();
    let mut map: HashMap<usize, usize> = numbers
        .into_iter()
        .take(len - 1)
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    let mut turn = map.len();

    while turn < 30_000_000 - 1 {
        let previous = map.entry(last).or_insert(turn);
        let distance = turn - *previous;
        *previous = turn;
        last = distance;
        turn += 1;
    }
    last.to_string()
}

type Line = Vec<usize>;

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(tag(","), digit1)(input)?;
    let line = line.into_iter().map(|s| s.parse().unwrap()).collect();
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0,3,6";
        let result = process_part1(input);
        assert_eq!(result, "436");
        let input = "1,3,2";
        assert_eq!(process_part1(input), "1");
        let input = "2,1,3";
        assert_eq!(process_part1(input), "10");
        let input = "1,2,3";
        assert_eq!(process_part1(input), "27");
        let input = "2,3,1";
        assert_eq!(process_part1(input), "78");
        let input = "3,2,1";
        assert_eq!(process_part1(input), "438");
        let input = "3,1,2";
        assert_eq!(process_part1(input), "1836");
    }

    #[test]
    fn part2_1() {
        let input = "0,3,6";
        assert_eq!(process_part2(input), "175594");
    }

    #[test]
    fn part2_2() {
        let input = "1,3,2";
        assert_eq!(process_part2(input), "2578");
    }

    #[test]
    fn part2_3() {
        let input = "2,1,3";
        assert_eq!(process_part2(input), "3544142");
    }

    #[test]
    fn part2_4() {
        let input = "1,2,3";
        assert_eq!(process_part2(input), "261214");
    }

    #[test]
    fn part2_5() {
        let input = "2,3,1";
        assert_eq!(process_part2(input), "6895259");
    }

    #[test]
    fn part2_6() {
        let input = "3,2,1";
        assert_eq!(process_part2(input), "18");
    }

    #[test]
    fn part2_7() {
        let input = "3,1,2";
        assert_eq!(process_part2(input), "362");
    }
}
