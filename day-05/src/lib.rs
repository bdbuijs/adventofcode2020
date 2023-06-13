use nom::{
    branch::alt,
    character::complete::newline,
    multi::{many_m_n, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let max = get_ids(lines).into_iter().max().unwrap();
    max.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    let mut ids = get_ids(lines);
    ids.sort();
    let mut current = ids[0];
    for id in ids {
        if id != current {
            return current.to_string();
        }
        current += 1;
    }
    unreachable!()
}

fn get_ids(lines: Vec<(Vec<char>, Vec<char>)>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|(row_dirs, seat_dirs)| {
            let (row, _) = row_dirs
                .into_iter()
                .fold((0_u32, 128_u32), |(row, size), c| match c {
                    'F' => (row, size / 2),
                    'B' => (row + size / 2, size / 2),
                    _ => unreachable!(),
                });
            let (seat, _) = seat_dirs
                .into_iter()
                .fold((0_u32, 8_u32), |(seat, size), c| match c {
                    'L' => (seat, size / 2),
                    'R' => (seat + size / 2, size / 2),
                    _ => unreachable!(),
                });
            let id = row * 8 + seat;
            id
        })
        .collect()
}

type Line<'a> = (Vec<char>, Vec<char>);

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, row) = parse_row(input)?;
    let (input, seat) = parse_seat(input)?;
    let line = (row, seat);
    Ok((input, line))
}

fn parse_row(input: &str) -> IResult<&str, Vec<char>> {
    let (input, directions) = many_m_n(
        0,
        7,
        alt((
            nom::character::complete::char('F'),
            nom::character::complete::char('B'),
        )),
    )(input)?;
    Ok((input, directions))
}

fn parse_seat(input: &str) -> IResult<&str, Vec<char>> {
    let (input, directions) = many_m_n(
        0,
        7,
        alt((
            nom::character::complete::char('R'),
            nom::character::complete::char('L'),
        )),
    )(input)?;
    Ok((input, directions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        let result = process_part1(input);
        assert_eq!(result, "820");
    }

    #[test]
    fn part2() {
        let input = "BFFFFFFLLL
BFFFFFFLLR
BFFFFFFLRR
BFFFFFFRLL
BFFFFFFRLR";
        let result = process_part2(input);
        assert_eq!(result, "514");
    }
}
