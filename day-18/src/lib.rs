use nom::{
    branch::alt,
    character::complete::{digit1, newline, one_of, space0},
    combinator::map_res,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, equations) = parse_input(input).unwrap();
    equations
        .into_iter()
        .map(|e| e.eval())
        .sum::<isize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let input = parenthesise_input(input);
    let d = parse_input(&input);
    let (_, equations) = d.unwrap();
    equations
        .into_iter()
        .map(|e| e.eval())
        .sum::<isize>()
        .to_string()
}

#[derive(Debug, Clone)]
enum Equation {
    Mul(Box<Equation>, Box<Equation>),
    Add(Box<Equation>, Box<Equation>),
    Int(isize),
}

impl Equation {
    fn new(op: char, left: Self, right: Self) -> Self {
        match op {
            '+' => Self::Add(Box::from(left), Box::from(right)),
            '*' => Self::Mul(Box::from(left), Box::from(right)),
            _ => unimplemented!(),
        }
    }

    fn eval(&self) -> isize {
        match self {
            &Self::Int(x) => x,
            Self::Mul(a, b) => a.eval() * b.eval(),
            Self::Add(a, b) => a.eval() + b.eval(),
        }
    }
}

// replace + )+(
// replace * ))*((
// add (( at the beginning of each expression and after each left parenthesis in the original expression; and
// add )) at the end of the expression and before each right parenthesis in the original expression.

fn parenthesise_input(input: &str) -> String {
    // dbg!(input);
    let mut result = String::new();
    result.push_str("((");
    input.chars().for_each(|c| match c {
        '+' => result.push_str(") + ("),
        '*' => result.push_str(")) * (("),
        '(' => result.push_str("((("),
        ')' => result.push_str(")))"),
        ' ' => {}
        other => result.push(other),
    });
    result.push_str("))");
    assert_eq!(
        result.chars().filter(|&c| c == '(').count(),
        result.chars().filter(|&c| c == ')').count()
    );
    result
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    let (input, lines) = separated_list1(newline, parse_equation)(input)?;
    Ok((input, lines))
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    // println!("Parsing equation:{}", input);
    let (input, mut lhs) = parse_term(input)?;
    // println!("About to break?");
    let (input, mut op) = one_of("+*")(input)?;
    let (input, _) = space0(input)?;
    let (mut input, mut rhs) = parse_term(input)?;
    while !input.is_empty() && !input.starts_with(')') && !input.starts_with('\n') {
        lhs = Equation::new(op, lhs.clone(), rhs.clone());
        (input, op) = one_of("+*")(input)?;
        (input, _) = space0(input)?;
        (input, rhs) = parse_term(input)?;
    }
    let eq = Equation::new(op, lhs, rhs);
    // println!("Parsed equation sucessfully! '{:?}' input:{}", eq, input);
    Ok((input, eq))
}

fn parse_term(input: &str) -> IResult<&str, Equation> {
    // println!("Parsing term:{}", input);
    alt((parse_int, parse_parens))(input)
}

fn parse_int(input: &str) -> IResult<&str, Equation> {
    // println!("Parsing int:{}", input);
    let (input, n) = map_res(digit1, |digits: &str| digits.parse())(input)?;
    let (input, _) = space0(input)?;
    // println!("Parsed int successfuly! '{}' input:{}", n, input);
    Ok((input, Equation::Int(n)))
}

fn parse_parens(input: &str) -> IResult<&str, Equation> {
    // println!("Parsing parens:{}", input);
    let (input, eq) = delimited(
        nom::character::complete::char('('),
        alt((parse_equation, parse_term)),
        nom::character::complete::char(')'),
    )(input)?;
    let (input, _) = space0(input)?;
    // println!("Parsed parens successfuly! '{:?}' input:{}", eq, input);
    Ok((input, eq))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let result = process_part1(input);
        assert_eq!(result, "71");
    }

    #[test]
    fn part1_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let result = process_part1(input);
        assert_eq!(result, "51");
    }

    #[test]
    fn part1_3() {
        let input = "2 * 3 + (4 * 5)";
        let result = process_part1(input);
        assert_eq!(result, "26");
    }

    #[test]
    fn part1_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let result = process_part1(input);
        assert_eq!(result, "437");
    }

    #[test]
    fn part1_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let result = process_part1(input);
        assert_eq!(result, "12240");
    }

    #[test]
    fn part1_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let result = process_part1(input);
        assert_eq!(result, "13632");
    }

    #[test]
    fn part2_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let result = process_part2(input);
        assert_eq!(result, "231");
    }

    #[test]
    fn part2_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let result = process_part2(input);
        assert_eq!(result, "51");
    }

    #[test]
    fn part2_3() {
        let input = "2 * 3 + (4 * 5)";
        let result = process_part2(input);
        assert_eq!(result, "46");
    }

    #[test]
    fn part2_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let result = process_part2(input);
        assert_eq!(result, "1445");
    }

    #[test]
    fn part2_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let result = process_part2(input);
        assert_eq!(result, "669060");
    }

    #[test]
    fn part2_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let result = process_part2(input);
        assert_eq!(result, "23340");
    }
}
