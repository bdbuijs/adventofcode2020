use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult};

pub fn process_part1(input: &str) -> String {
    let (_, (timestamp, buses)) = parse_input(input).unwrap();
    let buses: Vec<i32> = buses
        .into_iter()
        .filter_map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse().unwrap())
            }
        })
        .collect();
    let best = buses
        .into_iter()
        .map(|b| (b - (timestamp % b), b))
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    (best.0 * best.1).to_string()
}

#[allow(non_snake_case)]
pub fn process_part2(input: &str) -> String {
    let (_, (_, buses)) = parse_input(input).unwrap();
    let schedule = buses
        .into_iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if s == "x" {
                None
            } else {
                Some((idx as i128, s.parse().unwrap()))
            }
        })
        .collect::<Vec<(i128, i128)>>();

    // make sure CRT applies
    schedule
        .iter()
        .map(|&(_, i)| i)
        .combinations(2)
        .for_each(|comb| {
            assert_eq!(gcd(comb[0], comb[1]), 1);
        });

    let m = schedule.iter().map(|&(_, i)| i).product::<i128>();

    let result = m - schedule
        .into_iter()
        .map(|(a_i, m_i)| {
            let M_i = m / m_i;
            let y_i = multiplicative_inverse(M_i, m_i);
            a_i * M_i * y_i
        })
        .sum::<i128>()
        % m;

    result.to_string()
}

#[allow(dead_code)]
fn gcd(mut u: i128, mut v: i128) -> i128 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    // Using identity 2
    let shift = (u | v).trailing_zeros();
    // Make u odd
    u >>= u.trailing_zeros();

    loop {
        // Make v odd
        v >>= v.trailing_zeros();

        // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
        v -= u;
        let m = v >> 31;
        u += v & m;
        v = (v + m) ^ m;

        if v == 0 {
            break;
        }
    }

    u << shift
}

fn extended_euclidean(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
        (t0, t1) = (t1, t0 - q * t1);
    }

    // (gcd, bezout_a, bezout_b)
    (r0, s0, t0)
}

fn multiplicative_inverse(number: i128, modulus: i128) -> i128 {
    let (_g, s, _t) = extended_euclidean(number, modulus);
    (s % modulus + modulus) % modulus
}

fn parse_input(input: &str) -> IResult<&str, (i32, Vec<&str>)> {
    let (input, timestamp) = digit1(input)?;
    let timestamp = timestamp.parse().unwrap();
    let (input, _) = tag("\n")(input)?;
    let (input, buses) = separated_list1(tag(","), nom::branch::alt((tag("x"), digit1)))(input)?;
    Ok((input, (timestamp, buses)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "939
7,13,x,x,59,x,31,19";
        let result = process_part1(input);
        assert_eq!(result, "295");
    }

    #[test]
    fn part2_1() {
        let input = "939
7,13,x,x,59,x,31,19";
        let result = process_part2(input);
        assert_eq!(result, "1068781");
    }

    #[test]
    fn part2_2() {
        let input = "939
17,x,13,19";
        let result = process_part2(input);
        assert_eq!(result, "3417");
    }

    #[test]
    fn part2_3() {
        let input = "939
67,7,59,61";
        let result = process_part2(input);
        assert_eq!(result, "754018");
    }
    #[test]
    fn part2_4() {
        let input = "939
67,x,7,59,61";
        let result = process_part2(input);
        assert_eq!(result, "779210");
    }

    #[test]
    fn part2_5() {
        let input = "939
67,7,x,59,61";
        let result = process_part2(input);
        assert_eq!(result, "1261476");
    }

    #[test]
    fn part2_6() {
        let input = "939
1789,37,47,1889";
        let result = process_part2(input);
        assert_eq!(result, "1202161486");
    }

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(24, 36), 12);
    }

    #[test]
    fn crt_applies_to_input() {
        let input = include_str!("../input.txt");
        let (_, (_, v)) = parse_input(input).unwrap();
        let v: Vec<usize> = v
            .into_iter()
            .filter_map(|x| {
                if x == "x" {
                    None
                } else {
                    Some(x.parse().unwrap())
                }
            })
            .collect();
        v.into_iter().combinations(2).for_each(|comb| {
            assert_eq!(gcd(comb[0] as i128, comb[1] as i128), 1);
        })
    }

    #[test]
    fn extended_euclidean_works() {
        let (g, a, b) = extended_euclidean(84, 30);
        assert_eq!((g, a, b), (6, -1, 3));
    }
}
