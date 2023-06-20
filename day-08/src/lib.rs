use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::space1,
        complete::{digit1, newline},
    },
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, ops) = parse_input(input).unwrap();
    if let Err(val) = run(&ops) {
        val.to_string()
    } else {
        "".to_string()
    }
}

pub fn process_part2(input: &str) -> String {
    let (_, ops) = parse_input(input).unwrap();
    let mut idx_to_change = 1_usize;

    let result = loop {
        let new_instruction = loop {
            break match ops[idx_to_change] {
                Op::Acc(_) => {
                    idx_to_change += 1;
                    continue;
                }
                Op::Jmp(v) => Op::Nop(v),
                Op::Nop(v) => Op::Jmp(v),
            };
        };
        let mut potential_fix = ops.clone();
        potential_fix[idx_to_change] = new_instruction;
        if let Ok(val) = run(&potential_fix) {
            break val;
        }
        idx_to_change += 1;
    };
    result.to_string()
}

fn run(program: &Vec<Op>) -> Result<i32, i32> {
    let mut seen = vec![false; program.len()];
    let mut pc = 0_usize;
    let mut acc = 0;
    let end_of_program = program.len();
    loop {
        if pc == end_of_program {
            return Ok(acc);
        }
        if seen[pc] {
            return Err(acc);
        }
        let op = program[pc];
        seen[pc] = true;
        match op {
            Op::Nop(_) => pc += 1,
            Op::Acc(v) => {
                acc += v;
                pc += 1
            }
            Op::Jmp(v) => pc = (pc as i32 + v) as usize,
        }
    }
}

type Line = Op;

#[derive(Debug, Copy, Clone)]
enum Op {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, op) = alt((tag("nop"), tag("jmp"), tag("acc")))(input)?;
    let (input, _) = space1(input)?;
    let (input, plm) = alt((tag("+"), tag("-")))(input)?;
    let (input, val) = digit1(input)?;
    let plm = match plm {
        "+" => 1,
        "-" => -1,
        _ => unreachable!(),
    };
    let val = plm * val.parse::<i32>().unwrap();
    let op = match op {
        "nop" => Op::Nop(val),
        "jmp" => Op::Jmp(val),
        "acc" => Op::Acc(val),
        _ => unreachable!(),
    };
    Ok((input, op))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let result = process_part1(input);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let result = process_part2(input);
        assert_eq!(result, "8");
    }
}
