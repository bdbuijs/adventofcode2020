use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, mut waiting_area) = parse_input(input).unwrap();
    let xlen = waiting_area[0].len();
    let ylen = waiting_area.len();
    let mut neighbours = vec![vec![Vec::new(); xlen]; ylen];
    (0..ylen).for_each(|y| {
        (0..xlen).for_each(|x| {
            neighbours[y][x] = get_neighbours(xlen, ylen, x, y);
        })
    });
    let occupied_seats = loop {
        let mut new_waiting_area = waiting_area.clone();
        (0..ylen).for_each(|y| {
            (0..xlen).for_each(|x| {
                match (
                    waiting_area[y][x],
                    neighbours[y][x]
                        .iter()
                        .map(|&(xn, yn)| waiting_area[yn][xn].occupied())
                        .sum(),
                ) {
                    (Spot::Empty, 0) => new_waiting_area[y][x] = Spot::Occupied,
                    (Spot::Occupied, count) if count > 3 => new_waiting_area[y][x] = Spot::Empty,
                    _ => {}
                }
            })
        });

        if waiting_area == new_waiting_area {
            break new_waiting_area
                .into_iter()
                .flat_map(|v| v.into_iter())
                .map(|s| match s {
                    Spot::Occupied => 1,
                    _ => 0,
                })
                .sum::<usize>();
        }

        waiting_area = new_waiting_area;
    };
    occupied_seats.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut waiting_area) = parse_input(input).unwrap();
    let xlen = waiting_area[0].len();
    let ylen = waiting_area.len();
    let mut neighbours = vec![vec![Vec::new(); xlen]; ylen];
    (0..ylen).for_each(|y| {
        (0..xlen).for_each(|x| {
            neighbours[y][x] = get_neighbours_part2(&waiting_area, x, y);
        })
    });
    let occupied_seats = loop {
        let mut new_waiting_area = waiting_area.clone();
        (0..ylen).for_each(|y| {
            (0..xlen).for_each(|x| {
                match (
                    waiting_area[y][x],
                    neighbours[y][x]
                        .iter()
                        .map(|&(xn, yn)| waiting_area[yn][xn].occupied())
                        .sum(),
                ) {
                    (Spot::Empty, 0) => new_waiting_area[y][x] = Spot::Occupied,
                    (Spot::Occupied, count) if count > 4 => new_waiting_area[y][x] = Spot::Empty,
                    _ => {}
                }
            })
        });

        if waiting_area == new_waiting_area {
            break new_waiting_area
                .into_iter()
                .flat_map(|v| v.into_iter())
                .map(|s| match s {
                    Spot::Occupied => 1,
                    _ => 0,
                })
                .sum::<usize>();
        }

        waiting_area = new_waiting_area;
    };
    occupied_seats.to_string()
}

fn _print_area(area: &[Vec<Spot>]) {
    area.iter().for_each(|v| {
        println!(
            "{:?}",
            v.iter().cloned().map(|s| s.char()).collect::<String>()
        );
    })
}

fn get_neighbours(
    grid_width: usize,
    grid_height: usize,
    cell_x: usize,
    cell_y: usize,
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    let offsets = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (offset_x, offset_y) in offsets {
        let neighbor_x = cell_x as isize + offset_x;
        let neighbor_y = cell_y as isize + offset_y;

        if neighbor_x >= 0
            && neighbor_x < grid_width as isize
            && neighbor_y >= 0
            && neighbor_y < grid_height as isize
        {
            neighbours.push((neighbor_x as usize, neighbor_y as usize));
        }
    }

    neighbours
}

fn get_neighbours_part2(area: &[Vec<Spot>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let ymax = area.len() - 1;
    let xmax = area.first().unwrap().len() - 1;
    let mut neighbours = Vec::new();

    // up left
    let (mut newx, mut newy) = (x, y);
    while newx > 0 && newy > 0 {
        newx -= 1;
        newy -= 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // up
    let (newx, mut newy) = (x, y);
    while newy > 0 {
        newy -= 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // up right
    let (mut newx, mut newy) = (x, y);
    while newx < xmax && newy > 0 {
        newx += 1;
        newy -= 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // left
    let (mut newx, newy) = (x, y);
    while newx > 0 {
        newx -= 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // right
    let (mut newx, newy) = (x, y);
    while newx < xmax {
        newx += 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // down left
    let (mut newx, mut newy) = (x, y);
    while newx > 0 && newy < ymax {
        newx -= 1;
        newy += 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // down
    let (newx, mut newy) = (x, y);
    while newy < ymax {
        newy += 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    // down right
    let (mut newx, mut newy) = (x, y);
    while newx < xmax && newy < ymax {
        newx += 1;
        newy += 1;
        match area[newy][newx] {
            Spot::Floor => continue,
            _ => neighbours.push((newx, newy)),
        }
        break;
    }

    neighbours
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    Empty,
    Occupied,
    Floor,
}

impl Spot {
    fn occupied(&self) -> usize {
        match self {
            Self::Occupied => 1,
            _ => 0,
        }
    }

    #[allow(dead_code)]
    fn char(&self) -> char {
        match self {
            Self::Empty => 'L',
            Self::Occupied => '#',
            Self::Floor => '.',
        }
    }
}

type Line = Vec<Spot>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(parse_spot)(input)?;
    Ok((input, line))
}

fn parse_spot(input: &str) -> IResult<&str, Spot> {
    let (input, spot_char) = one_of(".#L")(input)?;
    let spot = match spot_char {
        '.' => Spot::Floor,
        'L' => Spot::Empty,
        '#' => Spot::Occupied,
        _ => {
            return IResult::Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            )))
        }
    };
    Ok((input, spot))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let result = process_part1(input);
        assert_eq!(result, "37");
    }

    #[test]
    fn part2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let result = process_part2(input);
        assert_eq!(result, "26");
    }
}
