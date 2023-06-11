use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline, space1},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, passwords) = parse_input(input).unwrap();
    passwords
        .into_iter()
        .filter_map(|(start, end, character, password)| {
            let count = password.matches(character).count() as u32;
            if (start..=end).contains(&count) {
                Some(true)
            } else {
                None
            }
        })
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    // this only works because it's ASCII input, otherwise as_bytes wouldn't index properly
    let (_, passwords) = parse_input(input).unwrap();
    passwords
        .into_iter()
        .filter_map(|(a, b, character, password)| {
            let password = password.as_bytes();
            let character = character as u8;
            let (a, b) = (a as usize, b as usize);
            if (password[a - 1] == character) ^ (password[b - 1] == character) {
                Some(true)
            } else {
                None
            }
        })
        .count()
        .to_string()
}
fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32, char, &str)>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, (u32, u32, char, &str)> {
    let (input, start) = nom::character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, character) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;

    Ok((input, (start, end, character, password)))
}

// fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
//     let (input, start) = nom::character::complete::u32(input)?;
//     let (input, _) = nom::bytes::complete::tag("-")(input)?;
//     let (input, end) = nom::character::complete::u32(input)?;
//     Ok((input, start..=end))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let input = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";
        let result = process_part2(input);
        assert_eq!(result, "1");
    }
}
