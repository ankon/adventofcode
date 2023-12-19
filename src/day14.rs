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
            Direction::North => self.tilt_north_south(0..rows, |r| Box::new(r+1..rows)),
            Direction::West => self.tilt_west_east(0..cols, |c| Box::new(c+1..cols)),
            Direction::South => self.tilt_north_south((0..rows).rev(), |r| Box::new((0..r).rev())),
            Direction::East => self.tilt_west_east((0..cols).rev(), |c| Box::new((0..c).rev())),
        }
    }

    pub fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    pub fn cycle_n(&mut self, n: usize) {
        // Eventually things cycle back a previous state. It seems that happens
        // after a few 10 cycles, so keep track of them in a simple vector.
        // As soon as we found it, we can fast-forward the cycle counter, and then
        // complete the remaining cycles.
        let mut previous_states = vec![self.checksum()];
        for cycle in 0..n {
            self.cycle();

            let checksum = self.checksum();
            println!("cycle {} checksum = {}", cycle, checksum);
            for (i, previous_checksum) in previous_states.iter().rev().enumerate() {
                if checksum == *previous_checksum {
                    println!("cycle {} matches cycle {} ({} cycles ago)", cycle, cycle-i, i);
                    let cycle_length = i + 1;
                    let remaining_cycles = n - cycle - 1;
                    println!("remaining cycles = {}, cycle length = {}", remaining_cycles, cycle_length);
                    let remaining_cycles = remaining_cycles % cycle_length;
                    println!("fast-forwarding and finishing the last {} cycles", remaining_cycles);
                    for _ in 0..remaining_cycles {
                        self.cycle();
                    }
                    return;
                }
            }
            previous_states.push(checksum);
        }
    }

    fn tilt_north_south(&mut self, rows: impl std::iter::Iterator<Item = usize>, source_rows: impl Fn(usize) -> Box<dyn std::iter::Iterator<Item = usize>>) {
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

    fn tilt_west_east(&mut self, cols: impl std::iter::Iterator<Item = usize>, source_cols: impl Fn(usize) -> Box<dyn std::iter::Iterator<Item = usize>>) {
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
            platform.cycle_n(CYCLES-1);
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

    static AFTER_WEST: &str = "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";

    static AFTER_SOUTH: &str = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";

    static AFTER_EAST: &str = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

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
        println!("initial\n{}", platform);
        platform.tilt(Direction::North);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.to_string().trim_end(), AFTER_NORTH);
    }

    #[test]
    fn test_tilt_west() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        println!("initial\n{}", platform);
        platform.tilt(Direction::West);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.to_string().trim_end(), AFTER_WEST);
    }

    #[test]
    fn test_tilt_south() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        println!("initial\n{}", platform);
        platform.tilt(Direction::South);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.to_string().trim_end(), AFTER_SOUTH);
    }

    #[test]
    fn test_tilt_east() {
        let mut platform: Platform = INITIAL.parse().unwrap();
        println!("initial\n{}", platform);
        platform.tilt(Direction::East);
        println!("after tilt\n{}", platform);
        assert_eq!(platform.to_string().trim_end(), AFTER_EAST);
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
        platform.cycle_n(CYCLES);
        println!("after cycle\n{}", platform);
        assert_eq!(platform.total_load(Direction::North), 64);
    }
}
