use nom::{
    character::complete::{digit1, newline, one_of},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut ferry = Ferry::new();
    instructions.into_iter().for_each(|a| ferry.navigate(a));
    ferry.manhattan().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, instructions) = parse_input(input).unwrap();
    let mut ferry = Ferry2::new();
    instructions.into_iter().for_each(|a| ferry.navigate(a));
    dbg!(&ferry);
    ferry.manhattan().to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self, amount: i32) -> Self {
        let mut dir = i32::from(self.clone());
        dir += amount + 360;
        dir.into()
    }
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        if value % 90 != 0 {
            panic!("Invalid amount of rotation: {:?}", value);
        }
        match (value + 360) % 360 {
            0 => Self::East,
            90 => Self::North,
            180 => Self::West,
            270 => Self::South,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::East => 0,
            Direction::North => 90,
            Direction::West => 180,
            Direction::South => 270,
        }
    }
}

#[derive(Debug)]
struct Ferry {
    east: i32,
    north: i32,
    heading: Direction,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            east: 0,
            north: 0,
            heading: Direction::East,
        }
    }

    fn forward(&self, x: i32) -> Action {
        match self.heading {
            Direction::North => Action::North(x),
            Direction::South => Action::South(x),
            Direction::East => Action::East(x),
            Direction::West => Action::West(x),
        }
    }

    fn navigate(&mut self, action: Action) {
        match action {
            Action::North(x) => self.north += x,
            Action::South(x) => self.north -= x,
            Action::East(x) => self.east += x,
            Action::West(x) => self.east -= x,
            Action::Forward(x) => self.navigate(self.forward(x)),
            Action::Left(x) => self.heading = self.heading.left(x),
            Action::Right(x) => self.heading = self.heading.left(-x),
        }
    }

    fn manhattan(&self) -> i32 {
        self.east.abs() + self.north.abs()
    }
}

#[derive(Debug)]
struct Waypoint {
    east: i32,
    north: i32,
}

impl Waypoint {
    fn rotate(&mut self, amount: i32) {
        match (amount + 360) % 360 {
            0 => {}
            90 => (self.east, self.north) = (-self.north, self.east),
            180 => (self.east, self.north) = (-self.east, -self.north),
            270 => (self.east, self.north) = (self.north, -self.east),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Ferry2 {
    east: i32,
    north: i32,
    waypoint: Waypoint,
}

impl Ferry2 {
    fn new() -> Self {
        Ferry2 {
            east: 0,
            north: 0,
            waypoint: Waypoint { east: 10, north: 1 },
        }
    }

    fn forward(&mut self, amount: i32) {
        self.east += self.waypoint.east * amount;
        self.north += self.waypoint.north * amount;
    }

    fn navigate(&mut self, action: Action) {
        match action {
            Action::North(x) => self.waypoint.north += x,
            Action::South(x) => self.waypoint.north -= x,
            Action::East(x) => self.waypoint.east += x,
            Action::West(x) => self.waypoint.east -= x,
            Action::Forward(x) => self.forward(x),
            Action::Left(x) => self.waypoint.rotate(x),
            Action::Right(x) => self.waypoint.rotate(-x),
        }
    }

    fn manhattan(&self) -> i32 {
        self.east.abs() + self.north.abs()
    }
}

type Line = Action;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, action) = one_of("NSEWLRF")(input)?;
    let (input, amount) = digit1(input)?;
    let amount: i32 = amount.parse().unwrap();
    let action = match action {
        'N' => Action::North(amount),
        'S' => Action::South(amount),
        'E' => Action::East(amount),
        'W' => Action::West(amount),
        'F' => Action::Forward(amount),
        'L' => Action::Left(amount),
        'R' => Action::Right(amount),
        _ => {
            return IResult::Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            )))
        }
    };

    Ok((input, action))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert_eq!(Direction::North.left(90), Direction::West);
        assert_eq!(Direction::East.left(-180), Direction::West);
    }

    #[test]
    fn part1() {
        let input = "F10
N3
F7
R90
F11";
        let result = process_part1(input);
        assert_eq!(result, "25");
    }

    #[test]
    fn part2() {
        let input = "F10
N3
F7
R90
F11";
        let result = process_part2(input);
        assert_eq!(result, "286");
    }
}
