use std::{collections::HashMap, fmt::Debug};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, tiles_raw) = parse_input(input).unwrap();
    let mut tiles: HashMap<usize, Tile> = tiles_raw
        .into_iter()
        .map(|(id, v)| (id, Tile::new(id, v)))
        .collect();
    let dimension = (tiles.len() as f64).sqrt() as usize;

    let mut solution = Vec::new();
    if solve(&mut solution, &mut tiles, dimension) {
        let mut n = 1;
        let first = solution.first().unwrap();
        n *= first.first().unwrap().id;
        n *= first.last().unwrap().id;
        let last = solution.last().unwrap();
        n *= last.first().unwrap().id;
        n *= last.last().unwrap().id;
        format!("{}", n)
    } else {
        "".to_string()
    }
}

pub fn process_part2(input: &str) -> String {
    // parse tiles
    let (_, tiles_raw) = parse_input(input).unwrap();
    let mut tiles: HashMap<usize, Tile> = tiles_raw
        .into_iter()
        .map(|(id, v)| (id, Tile::new(id, v)))
        .collect();
    let dimension = (tiles.len() as f64).sqrt() as usize;

    // find solution
    let mut solution = Vec::new();
    solve(&mut solution, &mut tiles, dimension);

    // extract image
    let mut image: Vec<Vec<Pixel>> = vec![Vec::with_capacity(8 * dimension); 8 * dimension];
    solution
        .into_iter()
        .enumerate()
        .for_each(|(major_row, tile_row)| {
            tile_row.into_iter().for_each(|tile| {
                tile.variants[tile.variant.unwrap()]
                    .contents
                    .iter()
                    .enumerate()
                    .for_each(|(tile_row, tiles)| {
                        image[major_row * 8 + tile_row].extend(tiles.iter());
                    })
            })
        });

    // count pixels that are on
    let on_count: usize = image
        .iter()
        .flat_map(|r| r.iter())
        .map(|pixel| match pixel {
            Pixel::On => 1,
            _ => 0,
        })
        .sum();

    // create all rotations/reflections of image
    let len = image.len();
    let mut images = vec![image];
    (0..3).for_each(|_| {
        let old_image = images.last().unwrap();
        let mut new_image = vec![Vec::with_capacity(len); len];
        (0..len).for_each(|x| {
            (0..len).for_each(|y| {
                new_image[x].push(old_image[len - y - 1][x]);
            })
        });
        images.push(new_image)
    });
    (0..4).for_each(|i| {
        images.push(images[i].iter().cloned().rev().collect());
    });

    // detect monsters
    for image in images {
        let mut monster_tiles = 0;
        image.windows(3).for_each(|w| {
            w[0].windows(20)
                .zip(w[1].windows(20).zip(w[2].windows(20)))
                .for_each(|(top_row, (middle_row, bottom_row))| {
                    if top_row[18] == Pixel::On
                        && middle_row[0] == Pixel::On
                        && middle_row[5] == Pixel::On
                        && middle_row[6] == Pixel::On
                        && middle_row[11] == Pixel::On
                        && middle_row[12] == Pixel::On
                        && middle_row[17] == Pixel::On
                        && middle_row[18] == Pixel::On
                        && middle_row[19] == Pixel::On
                        && bottom_row[1] == Pixel::On
                        && bottom_row[4] == Pixel::On
                        && bottom_row[7] == Pixel::On
                        && bottom_row[10] == Pixel::On
                        && bottom_row[13] == Pixel::On
                        && bottom_row[16] == Pixel::On
                    {
                        monster_tiles += 15;
                    }
                })
        });
        if monster_tiles > 0 {
            return (on_count - monster_tiles).to_string();
        }
    }

    "NOT FOUND".to_string()
}

fn solve(solution: &mut Image, unused_tiles: &mut HashMap<usize, Tile>, dimension: usize) -> bool {
    if unused_tiles.is_empty() {
        return true;
    }
    let current_column = if let Some(row) = solution.last() {
        if row.len() == dimension {
            solution.push(Vec::new());
            0
        } else {
            row.len()
        }
    } else {
        solution.push(Vec::new());
        0
    };
    let current_row = solution.len() - 1;
    let ids_to_try: Vec<usize> = unused_tiles.keys().copied().collect();
    for id in ids_to_try {
        let mut tile = unused_tiles
            .remove(&id)
            .expect("Tile not yet tried should be in map!");
        for variant_index in 0..tile.variants.len() {
            if current_row > 0 {
                let tile_above = solution
                    .get(current_row - 1)
                    .unwrap()
                    .get(current_column)
                    .unwrap();
                if tile_above.variants[tile_above.variant.unwrap()].bottom
                    != tile.variants[variant_index].top
                {
                    continue;
                }
            }
            // check if matches to tile to the left
            if current_column > 0 {
                let tile_to_left = solution
                    .get(current_row)
                    .unwrap()
                    .get(current_column - 1)
                    .unwrap();
                if tile_to_left.variants[tile_to_left.variant.unwrap()].right
                    != tile.variants[variant_index].left
                {
                    continue;
                }
            }
            // potential match!
            let _ = tile.variant.insert(variant_index);
            solution.last_mut().unwrap().push(tile.clone());
            if solve(solution, unused_tiles, dimension) {
                return true;
            } else {
                solution.last_mut().unwrap().pop().unwrap();
                tile.variant.take();
            }
        }
        // println!("Tried all variants of {id}");
        tile.variant.take();
        unused_tiles.insert(id, tile);
    }
    if solution.last().unwrap().is_empty() {
        solution.pop();
    }
    false
}

type Image = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
    On,
    Off,
}

impl Pixel {
    fn char(&self) -> char {
        match self {
            Self::On => '#',
            Self::Off => '.',
        }
    }
}

#[derive(Clone, Hash)]
struct Tile {
    id: usize,
    variants: Vec<TileVariant>,
    variant: Option<usize>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile").field("id", &self.id).finish()
    }
}

impl Tile {
    fn new(id: usize, v: Vec<Vec<char>>) -> Self {
        let mut variants = vec![TileVariant::new(v)];
        (0..3).for_each(|_| {
            variants.push(variants.last().unwrap().rotate());
        });
        (0..3).for_each(|i| {
            variants.push(variants[i].flip_vertical());
        });
        Self {
            id,
            variants,
            variant: None,
        }
    }
}

fn _print_pixels(v: &[Pixel]) {
    println!("{}", v.iter().map(|p| p.char()).collect::<String>());
}

fn _print_tiles(v: &HashMap<usize, Tile>) {
    println!(
        "{}",
        v.keys().map(|t| format!(" {} ", t)).collect::<String>()
    )
}

#[derive(Clone, Hash)]
struct TileVariant {
    top: Vec<Pixel>,
    bottom: Vec<Pixel>,
    left: Vec<Pixel>,
    right: Vec<Pixel>,
    contents: Vec<Vec<Pixel>>,
}

impl Debug for TileVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = self.top.iter().map(|p| p.char()).collect::<String>();
        output.extend(
            self.left
                .iter()
                .map(|p| p.char())
                .zip(
                    self.contents
                        .iter()
                        .map(|row| row.iter().map(|p| p.char()).collect::<String>()),
                )
                .zip(self.right.iter().map(|p| p.char()))
                .map(|((l, center), r)| format!("\n{l}{center}{r}")),
        );
        output.push('\n');
        output.extend(self.bottom.iter().map(|p| p.char()));
        f.write_fmt(format_args!("{}", output))
    }
}

impl TileVariant {
    fn new(v: Vec<Vec<char>>) -> Self {
        let len = v.len();
        Self {
            top: v[0]
                .iter()
                .map(|cell| match cell {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => unreachable!(),
                })
                .collect(),
            bottom: v
                .last()
                .unwrap()
                .iter()
                .map(|cell| match cell {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => unreachable!(),
                })
                .collect(),
            left: v
                .iter()
                .map(|vc| match vc[0] {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => unreachable!(),
                })
                .collect(),
            right: v
                .iter()
                .map(|vc| match vc.last().unwrap() {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => unreachable!(),
                })
                .collect(),
            contents: v
                .into_iter()
                .skip(1)
                .take(len - 2)
                .map(|row| {
                    row.into_iter()
                        .skip(1)
                        .take(len - 2)
                        .map(|pixel| match pixel {
                            '.' => Pixel::Off,
                            '#' => Pixel::On,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn rotate(&self) -> Self {
        let len = self.contents.len();
        let mut contents = vec![Vec::with_capacity(len); len];
        (0..len).for_each(|x| {
            (0..len).for_each(|y| {
                contents[x].push(self.contents[len - y - 1][x]);
            })
        });
        Self {
            top: self.left.iter().cloned().rev().collect(),
            bottom: self.right.iter().cloned().rev().collect(),
            left: self.bottom.clone(),
            right: self.top.clone(),
            contents,
        }
    }

    // fn flip_horizontal(&self) -> Self {
    //     Self {
    //         top: self.top.iter().cloned().rev().collect(),
    //         bottom: self.bottom.iter().cloned().rev().collect(),
    //         left: self.right.clone(),
    //         right: self.left.clone(),
    //     }
    // }

    fn flip_vertical(&self) -> Self {
        Self {
            top: self.bottom.clone(),
            bottom: self.top.clone(),
            left: self.left.iter().cloned().rev().collect(),
            right: self.right.iter().cloned().rev().collect(),
            contents: self.contents.iter().cloned().rev().collect(),
        }
    }

    #[allow(dead_code)]
    fn print_contents(&self) {
        self.contents.iter().for_each(|row| {
            println!(
                "{}",
                row.iter().map(|pixel| { pixel.char() }).collect::<String>()
            );
        })
    }
}

type TileVec = (usize, Vec<Vec<char>>);

fn parse_input(input: &str) -> IResult<&str, Vec<TileVec>> {
    let (input, tiles) = separated_list1(pair(newline, newline), parse_tile)(input)?;
    Ok((input, tiles))
}

fn parse_tile(input: &str) -> IResult<&str, TileVec> {
    let (input, id_str) = delimited(tag("Tile "), digit1, tag(":\n"))(input)?;
    let id = id_str.parse().unwrap();
    let (input, tile) = separated_list1(newline, many1(one_of(".#")))(input)?;

    Ok((input, (id, tile)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let result = process_part1(input);
        assert_eq!(result, "20899048083289");
    }

    #[test]
    fn part2() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let result = process_part2(input);
        assert_eq!(result, "273");
    }
}
