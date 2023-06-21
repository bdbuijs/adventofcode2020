use std::collections::{HashMap, HashSet};

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, cubes) = parse_input(input).unwrap();
    let mut dimension: HashMap<Coords, Cube> = HashMap::from_iter(
        cubes
            .into_iter()
            .enumerate()
            .flat_map(|(y, v)| v.into_iter().enumerate().zip(std::iter::repeat(y)))
            .map(|((x, c), y)| (Coords::new(x as isize, y as isize, 0), c)),
    );
    (0..6).for_each(|_| {
        let coords_to_check: HashSet<Coords> =
            dimension.keys().flat_map(|c| c.neighbours()).collect();
        dimension = coords_to_check
            .into_iter()
            .map(|coord| {
                let cube = if let Some(c) = dimension.get(&coord) {
                    c.clone()
                } else {
                    Cube::default()
                };
                let active_neighbours = coord
                    .neighbours()
                    .map(|neighbour| {
                        if let Some(cube) = dimension.get(&neighbour) {
                            if cube.active {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                match cube.active {
                    true if !(2..=3).contains(&active_neighbours) => (coord, Cube::default()),
                    false if active_neighbours == 3 => (coord, Cube::active()),
                    _ => (coord, cube),
                }
            })
            .collect();
    });

    let total_active = dimension
        .into_values()
        .map(|v| if v.active { 1 } else { 0 })
        .sum::<usize>();

    total_active.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, cubes) = parse_input(input).unwrap();
    let mut dimension: HashMap<HyperCoords, Cube> = HashMap::from_iter(
        cubes
            .into_iter()
            .enumerate()
            .flat_map(|(y, v)| v.into_iter().enumerate().zip(std::iter::repeat(y)))
            .map(|((x, c), y)| (HyperCoords::new(x as isize, y as isize, 0, 0), c)),
    );
    (0..6).for_each(|_| {
        let coords_to_check: HashSet<HyperCoords> =
            dimension.keys().flat_map(|c| c.neighbours()).collect();
        dimension = coords_to_check
            .into_iter()
            .map(|coord| {
                let cube = if let Some(c) = dimension.get(&coord) {
                    c.clone()
                } else {
                    Cube::default()
                };
                let active_neighbours = coord
                    .neighbours()
                    .map(|neighbour| {
                        if let Some(cube) = dimension.get(&neighbour) {
                            if cube.active {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                match cube.active {
                    true if !(2..=3).contains(&active_neighbours) => (coord, Cube::default()),
                    false if active_neighbours == 3 => (coord, Cube::active()),
                    _ => (coord, cube),
                }
            })
            .collect();
    });

    let total_active = dimension
        .into_values()
        .map(|v| if v.active { 1 } else { 0 })
        .sum::<usize>();

    total_active.to_string()
}

#[derive(Debug, Default, Clone)]
struct Cube {
    active: bool,
}

impl Cube {
    fn active() -> Self {
        Self { active: true }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Coords {
    x: isize,
    y: isize,
    z: isize,
}

impl Coords {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        ((self.x - 1)..=(self.x + 1)).flat_map(move |x| {
            ((self.y - 1)..=(self.y + 1)).flat_map(move |y| {
                ((self.z - 1)..=(self.z + 1)).filter_map(move |z| {
                    let c = Self { x, y, z };
                    if &c == self {
                        None
                    } else {
                        Some(c)
                    }
                })
            })
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct HyperCoords {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl HyperCoords {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }

    fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        ((self.x - 1)..=(self.x + 1)).flat_map(move |x| {
            ((self.y - 1)..=(self.y + 1)).flat_map(move |y| {
                ((self.z - 1)..=(self.z + 1)).flat_map(move |z| {
                    ((self.w - 1)..=(self.w + 1)).filter_map(move |w| {
                        let c = Self { x, y, z, w };
                        if &c == self {
                            None
                        } else {
                            Some(c)
                        }
                    })
                })
            })
        })
    }
}

type Line<'a> = Vec<Cube>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(one_of(".#"))(input)?;
    let cubes = line
        .into_iter()
        .map(|c| match c {
            '.' => Cube { active: false },
            '#' => Cube { active: true },
            _ => unreachable!(),
        })
        .collect();
    Ok((input, cubes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords() {
        let n = Coords::default().neighbours().count();
        assert_eq!(n, 26)
    }

    #[test]
    fn hypercoords() {
        let n = HyperCoords::default().neighbours().count();
        assert_eq!(n, 80)
    }

    #[test]
    fn part1() {
        let input = ".#.
..#
###";
        let result = process_part1(input);
        assert_eq!(result, "112");
    }

    #[test]
    fn part2() {
        let input = ".#.
..#
###";
        let result = process_part2(input);
        assert_eq!(result, "848");
    }
}
