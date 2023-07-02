use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, tile_locations) = parse_input(input).unwrap();
    let mut floor = HashMap::new();
    tile_locations.into_iter().for_each(|location| {
        let mut tile = HexTile::default();
        location
            .into_iter()
            .for_each(|direction| tile.step(direction));
        let coords = tile.coords();
        floor.entry(coords).or_insert(tile).flip();
    });
    let count = floor.into_values().filter(|tile| tile.black).count();
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, tile_locations) = parse_input(input).unwrap();
    let mut floor = HashMap::new();
    tile_locations.into_iter().for_each(|location| {
        let mut tile = HexTile::default();
        location
            .into_iter()
            .for_each(|direction| tile.step(direction));
        let coords = tile.coords();
        floor.entry(coords).or_insert(tile).flip();
    });
    (0..100).for_each(|_| {
        process_floor(&mut floor);
    });
    let count = floor.into_values().filter(|tile| tile.black).count();
    count.to_string()
}

fn process_floor(floor: &mut HashMap<(i32, i32), HexTile>) {
    // make sure all neighbours of black tiles are present
    let neighbours = floor
        .values()
        .filter(|tile| tile.black)
        .flat_map(|tile| tile.neighbour_coords().into_iter())
        .collect::<HashSet<(i32, i32)>>();
    neighbours.into_iter().for_each(|(q, r)| {
        floor.entry((q, r)).or_insert_with(|| HexTile::new(q, r));
    });
    // find which ones need to be flipped
    let flips: Vec<_> = floor
        .iter()
        .filter_map(|(&k, v)| match (count_neighbours(v, floor), v.black) {
            (0, true) => Some(k),
            (2, false) => Some(k),
            (x, true) if x > 2 => Some(k),
            _ => None,
        })
        .collect();
    // flip those
    flips.into_iter().for_each(|coords| {
        floor.entry(coords).and_modify(|tile| tile.flip());
    });
}

fn count_neighbours(tile: &HexTile, floor: &HashMap<(i32, i32), HexTile>) -> usize {
    tile.neighbour_coords()
        .into_iter()
        .filter(|neighbour| {
            if let Some(n) = floor.get(neighbour) {
                n.black
            } else {
                false
            }
        })
        .count()
}

#[derive(Debug, PartialEq, Eq)]
struct HexTile {
    q: i32,
    r: i32,
    black: bool,
}

impl HexTile {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r, black: false }
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::East => {
                self.q += 1;
            }
            Direction::SouthEast => {
                self.r += 1;
            }
            Direction::SouthWest => {
                self.q -= 1;
                self.r += 1;
            }
            Direction::West => {
                self.q -= 1;
            }
            Direction::NorthWest => {
                self.r -= 1;
            }
            Direction::NorthEast => {
                self.q += 1;
                self.r -= 1;
            }
        }
    }

    fn flip(&mut self) {
        self.black = !self.black;
    }

    fn coords(&self) -> (i32, i32) {
        (self.q, self.r)
    }

    fn neighbour_coords(&self) -> [(i32, i32); 6] {
        [
            (self.q + 1, self.r),
            (self.q, self.r + 1),
            (self.q - 1, self.r + 1),
            (self.q - 1, self.r),
            (self.q, self.r - 1),
            (self.q + 1, self.r - 1),
        ]
    }
}

impl Default for HexTile {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

type Line = Vec<Direction>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_direction)(input)?;
    Ok((input, line))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        tag("e"),
        tag("se"),
        tag("sw"),
        tag("w"),
        tag("nw"),
        tag("ne"),
    ))(input)?;
    let direction = match dir {
        "e" => Direction::East,
        "se" => Direction::SouthEast,
        "sw" => Direction::SouthWest,
        "w" => Direction::West,
        "nw" => Direction::NorthWest,
        "ne" => Direction::NorthEast,
        _ => unreachable!(),
    };
    Ok((input, direction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let result = process_part1(input);
        assert_eq!(result, "10");
    }

    #[test]
    fn part2() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let result = process_part2(input);
        assert_eq!(result, "2208");
    }
}
