#[derive(Clone, Copy, Debug, PartialEq)]
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
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();
        match direction {
            Direction::North => self.tilt_north_south(0..=rows-1, |r| r+1..=rows-1),
            Direction::West => self.tilt_west_east(0..=cols-1, |c| c+1..=cols-1),
            Direction::South => self.tilt_north_south(rows-1..=0, |r| r-1..=0),
            Direction::East => self.tilt_west_east(cols-1..=0, |c| c-1..=0),
        }
    }

    pub fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt_north_south(&mut self, rows: std::ops::RangeInclusive<usize>, source_rows: impl Fn(usize) -> std::ops::RangeInclusive<usize>) {
        for r in rows {
            for c in 0..self.tiles[r].len() {
                if self.tiles[r][c] != Tile::Empty {
                    continue;
                }

                // Move the closest tile in South direction here.
                for r2 in source_rows(r) {
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

    fn tilt_west_east(&mut self, cols: std::ops::RangeInclusive<usize>, source_cols: impl Fn(usize) -> std::ops::RangeInclusive<usize>) {
        for c in cols {
            for r in 0..self.tiles.len() {
                if self.tiles[r][c] != Tile::Empty {
                    continue;
                }

                // Move the closest tile in South direction here.
                for c2 in source_cols(c) {
                    let other_tile = &mut self.tiles[r][c2];
                    match other_tile {
                        Tile::Empty => {},
                        Tile::FixedRock => {
                            // Abort: This rock won't move, and anything further south will get stuck here.
                            break;
                        },
                        Tile::RoundedRock => {
                            // This one!
                            self.tiles[r][c] = Tile::RoundedRock;
                            self.tiles[r][c2] = Tile::Empty;
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

    pub fn checksum(&self) -> String {
        let mut hasher = crypto::sha1::Sha1::new();
        for row in self.tiles.iter() {
            for tile in row.iter() {
                crypto::digest::Digest::input(&mut hasher, &[*tile as u8]);
            }
        }
        crypto::digest::Digest::result_str(&mut hasher)
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (_, row) in self.tiles.iter().enumerate() {
            for tile in row {
                write!(f, "{}", match tile {
                    Tile::Empty => '.',
                    Tile::FixedRock => '#',
                    Tile::RoundedRock => 'O',
                })?;
            }
            writeln!(f)?
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
    const CYCLES: usize = 1000000000;
    match std::fs::read_to_string("day14.input") {
        Ok(input) => {
            let mut platform: Platform = input.parse().unwrap();
            println!("initial\n{}", platform);
            platform.tilt(Direction::North);
            println!("after tilt\n{}", platform);
            println!("total load = {}", platform.total_load(Direction::North));

            // Complete the first cycle.
            platform.tilt(Direction::West);
            platform.tilt(Direction::South);
            platform.tilt(Direction::East);

            // Complete the remaining cycles
            let mut previous_checksum = platform.checksum();
            for cycle in 0..CYCLES-1 {
                platform.cycle();

                let checksum = platform.checksum();
                println!("cycle {} checksum = {}", cycle, checksum);
                if checksum == previous_checksum {
                    break;
                }
                previous_checksum = checksum;
                if cycle % 1000 == 0 {
                    println!("complete {}%", cycle/CYCLES*100);
                }
            }
            println!("total load after {} cycles = {}", CYCLES, platform.total_load(Direction::North));
        },
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static CYCLES: usize = 1000000000;
    static INITIAL: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    static AFTER_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    static AFTER_CYCLE_1: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    static AFTER_CYCLE_2: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

    static AFTER_CYCLE_3: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn test_part1() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        println!("initial\n{}", platform);
        platform.tilt(Direction::North);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.total_load(Direction::North), 136);
    }

    #[test]
    fn test_tilt_north() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        platform.tilt(Direction::North);
        assert_eq!(platform.to_string().trim_end(), AFTER_NORTH);
    }

    #[test]
    fn test_cycle() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        platform.cycle();
        assert_eq!(platform.to_string().trim_end(), AFTER_CYCLE_1);
        platform.cycle();
        assert_eq!(platform.to_string().trim_end(), AFTER_CYCLE_2);
        platform.cycle();
        assert_eq!(platform.to_string().trim_end(), AFTER_CYCLE_3);
    }

    #[test]
    fn test_part2() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        println!("initial\n{}", platform);

        let mut previous_checksum = platform.checksum();
        for cycle in 0..CYCLES {
            platform.cycle();

            let checksum = platform.checksum();
            println!("cycle {} checksum = {}", cycle, checksum);
            if checksum == previous_checksum {
                break;
            }
            previous_checksum = checksum;
        }
        println!("after tilt\n{}", platform);
        assert_eq!(platform.total_load(Direction::North), 64);
    }
}
