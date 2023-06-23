use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::char as nomchar,
        complete::{alpha1, digit1, newline, one_of, space1},
    },
    combinator::recognize,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use regex::Regex;

pub fn process_part1(input: &str) -> String {
    let (rest, (mut rule_strs, messages)) = parse_input(input).unwrap();
    assert!(rest.is_empty());
    rule_strs.sort_by(|(i_a, _), (i_b, _)| i_a.cmp(i_b));
    let mut regex_rule_strs: Vec<&str> = Vec::new();
    rule_strs.into_iter().for_each(|(i, rule)| {
        while regex_rule_strs.len() < i {
            regex_rule_strs.push("");
        }
        regex_rule_strs.push(rule)
    });
    let regex_rule_0 = parse_regex(&regex_rule_strs);
    let matches = messages
        .into_iter()
        .filter(|&message| regex_rule_0.is_match(message))
        .count();

    matches.to_string()
}

pub fn process_part2(input: &str) -> String {
    let new_input = input.replace("8: 42", "8: 42 | 8 42");
    let new_input = new_input.replace("11: 42 31", "11: 42 31 | 42 11 31");
    let input = &new_input;
    let (rest, (mut rule_strs, messages)) = parse_input(input).unwrap();
    assert!(rest.is_empty());
    rule_strs.sort_by(|(i_a, _), (i_b, _)| i_a.cmp(i_b));
    let mut rule_book: Vec<&str> = Vec::new();
    rule_strs.into_iter().for_each(|(i, rule)| {
        while rule_book.len() < i {
            rule_book.push("");
        }
        rule_book.push(rule)
    });
    let (rule31, rule42) = parse_regex2(&rule_book);
    let matches = messages
        .into_iter()
        .filter(|&message| {
            let mut pos = 0;
            let mut count42 = 0;
            let mut count31 = 0;
            while let Some(mtch) = rule42.find_at(message, pos) {
                if mtch.start() != pos {
                    break;
                }
                count42 += 1;
                pos = mtch.end();
            }
            while let Some(mtch) = rule31.find_at(message, pos) {
                if mtch.start() != pos {
                    break;
                }
                count31 += 1;
                pos = mtch.end();
            }

            (pos == message.len()) && (count31 > 0) && (count31 < count42)
        })
        .count();

    matches.to_string()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(usize, &str)>, Vec<&str>)> {
    let (input, rules_str) = take_until("\n\n")(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (rest, rules) = separated_list1(newline, parse_rule_str)(rules_str)?;
    assert!(rest.is_empty());
    let (input, messages) = separated_list1(newline, alpha1)(input)?;
    Ok((input, (rules, messages)))
}

fn parse_rule_str(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, (i, _, rule)) = tuple((
        digit1,
        tag(": "),
        alt((
            recognize(tuple((nomchar('"'), one_of("ab"), nomchar('"')))), // character
            recognize(tuple((digit1, space1, digit1, space1, digit1))),   // three rules
            recognize(delimited(
                tuple((digit1, space1, digit1)),
                tag(" | "),
                tuple((digit1, space1, digit1, space1, digit1)),
            )), // two rules | three rules
            recognize(delimited(
                tuple((digit1, space1, digit1)),
                tag(" | "),
                tuple((digit1, space1, digit1)),
            )), // two rules | two rules
            recognize(tuple((digit1, space1, digit1))),                   // two rules
            recognize(delimited(
                digit1,
                tag(" | "),
                tuple((digit1, space1, digit1)),
            )), // one rule | two rules
            recognize(delimited(digit1, tag(" | "), digit1)),             // one rule | one rule
            recognize(digit1),                                            // single rule
        )),
    ))(input)?;
    let i = i.parse::<usize>().unwrap();
    Ok((input, (i, rule)))
}

fn parse_recursive(rule_book: &Vec<&str>, r: usize) -> String {
    let rule_s = rule_book[r];
    if let IResult::<_, _>::Ok((_, c)) =
        delimited(nomchar('"'), recognize(one_of("ab")), nomchar('"'))(rule_s)
    {
        c.to_string()
    } else {
        let parts = rule_s.split(' ').collect::<Vec<_>>();
        let inner_part = parts
            .into_iter()
            .map(|p| match p {
                "|" => p.to_string(),
                s => parse_recursive(rule_book, s.parse().unwrap()),
            })
            .collect::<String>();
        format!("({})", inner_part)
    }
}

fn parse_regex(rule_book: &Vec<&str>) -> Regex {
    let regex_str = format!("^{}$", parse_recursive(rule_book, 0));
    Regex::new(&regex_str).unwrap()
}

fn parse_regex2(rule_book: &Vec<&str>) -> (Regex, Regex) {
    let rule_31_str = parse_recursive(rule_book, 31);
    let rule_31 = Regex::new(&rule_31_str).unwrap();
    let rule_42_str = parse_recursive(rule_book, 42);
    let rule_42 = Regex::new(&rule_42_str).unwrap();
    (rule_31, rule_42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let result1 = process_part1(input);
        assert_eq!(result1, "3");
        let result2 = process_part2(input);
        assert_eq!(result2, "12");
    }
}
