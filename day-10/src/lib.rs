use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, mut adapters) = parse_input(input).unwrap();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let mut ones = 0;
    let mut threes = 0;
    adapters.windows(2).for_each(|s| {
        let diff = s[1] - s[0];
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    });
    (ones * threes).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut adapters) = parse_input(input).unwrap();
    adapters.sort();
    adapters.insert(0, 0);
    let mut ways = vec![0_usize; *adapters.last().unwrap() + 3];
    let idx_last = ways.len() - 3;
    ways[idx_last] = 1;
    let mut adapters_iter = adapters.into_iter().rev();
    adapters_iter.next();

    for adapter in adapters_iter {
        let mut ways_from_here = 0;
        ((adapter + 1)..(adapter + 4)).for_each(|next_adapter| {
            ways_from_here += ways[next_adapter];
        });
        ways[adapter] = ways_from_here;
    }
    ways[0].to_string()
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
    fn part1_small() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let result = process_part1(input);
        assert_eq!(result, "35");
    }

    #[test]
    fn part1_large() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let result = process_part1(input);
        assert_eq!(result, "220");
    }

    #[test]
    fn part2_small() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let result = process_part2(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_large() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let result = process_part2(input);
        assert_eq!(result, "19208");
    }
}
