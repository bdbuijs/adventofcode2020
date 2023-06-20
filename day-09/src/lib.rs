use std::collections::BTreeSet;

use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str, previous: usize) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let mut set: BTreeSet<Line> = numbers[..previous].iter().cloned().collect();
    for idx in previous..numbers.len() {
        let n = numbers[idx];
        let mut found = false;
        for num1 in set.iter().cloned() {
            let res = n - num1;
            if set.contains(&res) {
                found = true;
                break;
            }
        }
        if found {
            set.remove(&numbers[idx - previous]);
            set.insert(n);
            continue;
        }
        return n.to_string();
    }
    "".to_string()
}

pub fn process_part2(input: &str, previous: usize) -> String {
    let (_, numbers) = parse_input(input).unwrap();
    let mut set: BTreeSet<Line> = numbers[..previous].iter().cloned().collect();
    let mut invalid_num = -1;
    for idx in previous..numbers.len() {
        let n = numbers[idx];
        let mut found = false;
        for num1 in set.iter().cloned() {
            let res = n - num1;
            if set.contains(&res) {
                found = true;
                break;
            }
        }
        if found {
            set.remove(&numbers[idx - previous]);
            set.insert(n);
            continue;
        }
        invalid_num = n;
        break;
    }
    assert_ne!(invalid_num, -1);
    dbg!(invalid_num);
    for start in 0..(numbers.len() - 1) {
        let mut end = start + 1;
        let mut sum = numbers[start] + numbers[end];
        while end < numbers.len() - 1 {
            if sum == invalid_num {
                end += 1;
                return (numbers[start..end].iter().min().unwrap()
                    + numbers[start..end].iter().max().unwrap())
                .to_string();
            }
            end += 1;
            sum += numbers[end];
        }
    }
    "".to_string()
}

type Line = i64;

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
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let result = process_part1(input, 5);
        assert_eq!(result, "127");
    }

    #[test]
    fn part2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let result = process_part2(input, 5);
        assert_eq!(result, "62");
    }
}
