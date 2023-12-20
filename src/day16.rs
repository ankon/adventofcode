#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty = '.' as isize,
    SplitHorizontal = '-' as isize,
    SplitVertical = '|' as isize,
    MirrorBottomTop = '/' as isize,
    MirrorTopBottom = '\\' as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Contraption {
    tiles: Vec<Vec<Tile>>,
}

impl std::str::FromStr for Contraption {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => Tile::Empty,
                    '-' => Tile::SplitHorizontal,
                    '|' => Tile::SplitVertical,
                    '/' => Tile::MirrorBottomTop,
                    '\\' => Tile::MirrorTopBottom,
                    _ => return Err("invalid character"),
                });
            }
            tiles.push(row);
        }
        Ok(Contraption { tiles })
    }
}

impl Contraption {
    pub fn simulate_light_beam(&self, print: bool, stop_after: Option<usize>) -> usize {
        let mut beam_heads: Vec<(usize, usize, Direction)> = vec![];

        // Create a field of the same size as the contraption and track the direction of each beam
        // that hit that tile.
        let mut field: Vec<Vec<Vec<Direction>>> = self.tiles.iter()
            .map(|row| {
                vec![0; row.len()].iter()
                    .map(|_| vec![])
                    .collect()
            }).collect();

        // Start with one beam in the top left corner, pointing right:
        let mut steps = 0;
        beam_heads.push((0, 0, Direction::Right));
        while let Some((x, y, direction)) = beam_heads.pop() {
            if print {
                println!("beam at ({}, {}) {:?}", x, y, direction);
            }
            // Add the direction of that beam to the field IFF it didn't exist yet.
            // If it did, we will certainly not produce anything new, and can drop this beam.
            if field[y][x].contains(&direction) {
                if print {
                    println!("beam already hit this tile");
                }
                continue;
            }
            field[y][x].push(direction);

            if print {
                self.print_field(&field);
            }

            // Evaluate the tile itself
            let mut new_beams = vec![];
            match self.tiles[y][x] {
                Tile::Empty => {
                    // Nothing happens, the beam continues in its direction.
                    new_beams.push(self.move_beam((x, y, direction)));
                },
                Tile::MirrorBottomTop => {
                    // The beam changes direction.
                    new_beams.push(self.move_beam((x, y, match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    })));
                },
                Tile::MirrorTopBottom => {
                    // The beam changes direction.
                    new_beams.push(self.move_beam((x, y, match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    })));
                },
                Tile::SplitHorizontal => {
                    // If the beam is currently vertical it splits into two beams, one going left and one going right,
                    // otherwise it passes through.
                    if direction == Direction::Up || direction == Direction::Down {
                        new_beams.push(self.move_beam((x, y, Direction::Left)));
                        new_beams.push(self.move_beam((x, y, Direction::Right)));
                    } else {
                        new_beams.push(self.move_beam((x, y, direction)));
                    }
                },
                Tile::SplitVertical => {
                    // If the beam is currently horizontal it splits into two beams, one going up and one going down,
                    // otherwise it passes through.
                    if direction == Direction::Left || direction == Direction::Right {
                        new_beams.push(self.move_beam((x, y, Direction::Up)));
                        new_beams.push(self.move_beam((x, y, Direction::Down)));
                    } else {
                        new_beams.push(self.move_beam((x, y, direction)));
                    }
                },
            }

            new_beams.drain(..)
                .filter_map(|b| {
                    if b.is_some() {
                        if print {
                            println!("new beam {:?}", b);
                        }
                        b
                    } else {
                        if print {
                            println!("beam lost")
                        }
                        None
                    }
                })
                .for_each(|b| beam_heads.push(b));

            if print {
                println!("active beams = {}", beam_heads.len());
            }

            steps += 1;
            if let Some(stop_after) = stop_after {
                if stop_after == steps {
                    break;
                }
            }
        }

        // Sum up all energized tiles, ignoring how many beams touched a single tile.
        field.iter().map(|row| row.iter().map(|directions| if directions.is_empty() { 0 } else { 1 }).sum::<usize>()).sum::<usize>()
    }

    fn move_beam(&self, beam: (usize, usize, Direction)) -> Option<(usize, usize, Direction)> {
        let (x, y, direction) = beam;
        match direction {
            Direction::Up => if (y as isize) - 1 < 0 { None } else { Some((x, y - 1, direction)) },
            Direction::Down => if y + 1 >= self.tiles.len() { None } else { Some((x, y + 1, direction)) },
            Direction::Left => if (x as isize) - 1 < 0 { None } else { Some((x - 1, y, direction)) },
            Direction::Right => if x + 1 >= self.tiles[0].len() { None } else { Some((x + 1, y, direction)) },
        }
    }

    fn print_field(&self, field: &[Vec<Vec<Direction>>]) {
        // Clear screen and move to top left corner
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for (y, row) in field.iter().enumerate() {
            for (x, directions) in row.iter().enumerate() {
                print!("{}", match self.tiles[y][x] {
                    Tile::Empty => {
                        match directions.len() {
                            0 => '.',
                            1 => match directions[0] {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            },
                            _ => 'X',
                        }
                    },
                    Tile::SplitHorizontal => '-',
                    Tile::SplitVertical => '|',
                    Tile::MirrorBottomTop => '/',
                    Tile::MirrorTopBottom => '\\',
                });
            }
            println!();
        }
    }
}

pub fn main() {
    match std::fs::read_to_string("day16.input") {
        Ok(input) => {
            if let Ok(contraption) = input.parse::<Contraption>() {
                println!("part1 = {}", contraption.simulate_light_beam(true, None));
            } else {
                println!("error parsing input");
            }
        },
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part1() {
        assert_eq!(EXAMPLE.parse::<Contraption>().unwrap().simulate_light_beam(true, Some(200)), 46);
    }
}
