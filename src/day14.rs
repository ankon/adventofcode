#[derive(Debug, PartialEq)]
enum Tile {
    Empty = '.' as isize,
    FixedRock = '#' as isize,
    RoundedRock = 'O' as isize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Platform {
    tiles: Vec<Vec<Tile>>,
}

impl Platform {
    pub fn tilt(&mut self, direction: Direction) {
        assert_eq!(direction, Direction::North);

        let rows = self.tiles.len();
        for r in 0..rows  {
            for c in 0..self.tiles[r].len() {
                if self.tiles[r][c] != Tile::Empty {
                    continue;
                }

                // Move the closest tile in South direction here.
                for r2 in r+1..rows {
                    let other_tile = &mut self.tiles[r2][c];
                    match other_tile {
                        Tile::Empty => {},
                        Tile::FixedRock => {
                            // Abort: This rock won't move, and anything further south will get stuck here.
                            break;
                        },
                        Tile::RoundedRock => {
                            // This one!
                            self.tiles[r][c] = Tile::RoundedRock;
                            self.tiles[r2][c] = Tile::Empty;
                            break;
                        },
                    }
                }
            }
        }
    }

    pub fn total_load(&self, beam: Direction) -> usize {
        assert_eq!(beam, Direction::North);

        let mut result = 0;
        let rows = self.tiles.len();
        for (r, row) in self.tiles.iter().enumerate() {
            for tile in row.iter() {
                if *tile == Tile::RoundedRock {
                    result += rows - r;
                }
            }
        }
        result
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rows = self.tiles.len();
        for (r, row) in self.tiles.iter().enumerate() {
            for tile in row {
                write!(f, "{}", match tile {
                    Tile::Empty => '.',
                    Tile::FixedRock => '#',
                    Tile::RoundedRock => 'O',
                })?;
            }
            writeln!(f, " {:3}", rows - r)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => Tile::Empty,
                    '#' => Tile::FixedRock,
                    'O' => Tile::RoundedRock,
                    _ => panic!("invalid tile"),
                });
            }
            tiles.push(row);
        }
        Ok(Platform { tiles })
    }
}

pub fn main() {
    match std::fs::read_to_string("day14.input") {
        Ok(input) => {
            let mut platform: Platform = input.parse().unwrap();
            println!("initial\n{}", platform);
            platform.tilt(Direction::North);
            println!("after tilt\n{}", platform);
            println!("total load = {}", platform.total_load(Direction::North));
        },
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut platform: Platform = DATA.parse().unwrap();
        println!("initial\n{}", platform);
        platform.tilt(Direction::North);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.total_load(Direction::North), 136);
    }
}
