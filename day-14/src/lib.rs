use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut mask = Instruction::Mask((0, 0, 0));
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(_) => mask = instruction,
            Instruction::Mem((addr, val)) => {
                let val = mask.apply_val(val).unwrap();
                memory.insert(addr, val);
            }
        }
    }
    memory.values().sum::<u64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut mask = Instruction::Mask((0, 0, 0));
    let mut memory = HashMap::new();
    instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            Instruction::Mask(_) => mask = instruction,
            Instruction::Mem((addr, val)) => {
                mask.apply_mem(addr).unwrap().into_iter().for_each(|add| {
                    memory.insert(add, val);
                });
            }
        });
    memory.values().sum::<u64>().to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mask((u64, u64, u64)),
    Mem((u64, u64)),
}

impl Instruction {
    fn apply_val(&self, value: u64) -> Result<u64, &str> {
        match self {
            Self::Mask((on, off, _)) => Ok((value | on) & off),
            Self::Mem(_) => Err("Attempted to apply Mem instruction to value!"),
        }
    }

    fn apply_mem(&self, address: u64) -> Result<Vec<u64>, &str> {
        let base = 1 << 63;
        match self {
            &Self::Mask((mask_on, _, mut mask_float)) => {
                let mut addresses = vec![address | mask_on];
                while mask_float > 0 {
                    let z = mask_float.leading_zeros() as u64;
                    let m_on = base >> z;
                    let m_off = u64::MAX ^ m_on;
                    addresses = addresses
                        .into_iter()
                        .flat_map(|add| [add | m_on, add & m_off].into_iter())
                        .collect();
                    mask_float &= m_off;
                }
                Ok(addresses)
            }
            Self::Mem(_) => Err("Attempted to apply Mem instruction to value!"),
        }
    }
}

type Line = Instruction;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = alt((parse_mask, parse_mem))(input)?;
    Ok((input, line))
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, mask) = many1(one_of("X10"))(input)?;
    let mut mask_on = 0_u64;
    let mut mask_off = 0_u64;
    let mut mask_float = 0_u64;
    mask.into_iter().for_each(|c| {
        mask_on <<= 1;
        mask_off <<= 1;
        mask_float <<= 1;
        mask_off |= 1;
        match c {
            'X' => mask_float |= 1,
            '1' => mask_on |= 1,
            '0' => mask_off &= u64::MAX - 1,
            _ => unreachable!(),
        }
    });
    Ok((input, Instruction::Mask((mask_on, mask_off, mask_float))))
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mem[")(input)?;
    let (input, address) = digit1(input)?;
    let address: u64 = address.parse().unwrap();
    let (input, _) = tag("] = ")(input)?;
    let (input, value) = digit1(input)?;
    let value: u64 = value.parse().unwrap();
    Ok((input, Instruction::Mem((address, value))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_works1() {
        let (_, instruction) = parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        if let Instruction::Mask((mask_on, mask_off, _)) = instruction {
            let a1 = (11_u64 | mask_on) & mask_off;
            let a2 = (101_u64 | mask_on) & mask_off;
            let a3 = mask_on & mask_off;
            assert_eq!(a1, 73);
            assert_eq!(a2, 101);
            assert_eq!(a3, 64);
        } else {
            panic!("Result is not a mask!");
        }
    }

    #[test]
    fn mask_works2_1() {
        let (_, instruction) = parse_mask("mask = 00000000000000000000000000000000X0XX").unwrap();
        assert_eq!(instruction, Instruction::Mask((0, 11, 11)));
        if let Instruction::Mask(_) = instruction {
            let mut v = instruction.apply_mem(26).unwrap();
            v.sort();
            assert_eq!(v, vec![16, 17, 18, 19, 24, 25, 26, 27]);
        } else {
            panic!("Result is not a mask!");
        }
    }

    #[test]
    fn mask_works2_2() {
        let (_, instruction) = parse_mask("mask = 000000000000000000000000000000X1001X").unwrap();
        assert_eq!(instruction, Instruction::Mask((18, 51, 33)));
        if let Instruction::Mask(_) = instruction {
            let mut v = instruction.apply_mem(42).unwrap();
            v.sort();
            assert_eq!(v, vec![26, 27, 58, 59]);
        } else {
            panic!("Result is not a mask!");
        }
    }

    #[test]
    fn part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let result = process_part1(input);
        assert_eq!(result, "165");
    }

    #[test]
    fn part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let result = process_part2(input);
        assert_eq!(result, "208");
    }
}
