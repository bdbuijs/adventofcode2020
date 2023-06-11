use nom::{
    branch::alt,
    character::complete::{newline, one_of},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, forest) = parse_input(input).unwrap();
    walk(&forest, 3, 1).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, forest) = parse_input(input).unwrap();
    let mut answer = 1;
    for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        answer *= walk(&forest, right, down);
    }
    answer.to_string()
}

fn walk(forest: &Vec<Line>, right: usize, down: usize) -> u32 {
    let rmod = forest[0].len();
    let depth = forest.len();
    let mut right_pos = 0;
    let mut down_pos = 0;
    let mut trees = 0;
    while down_pos < depth {
        if forest[down_pos][right_pos] {
            trees += 1;
        }
        right_pos = (right_pos + right) % rmod;
        down_pos += down;
    }
    trees
}

type Line = Vec<bool>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    Ok(many1(parse_tree)(input)?)
}

fn parse_tree(input: &str) -> IResult<&str, bool> {
    Ok(alt((
        value(true, nom::character::complete::char('#')),
        value(false, nom::character::complete::char('.')),
    ))(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let result = process_part1(input);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let result = process_part2(input);
        assert_eq!(result, "336");
    }
}
