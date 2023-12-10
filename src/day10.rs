#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn x_offset(&self) -> isize {
        match self {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        }
    }

    pub fn y_offset(&self) -> isize {
        match self {
            Direction::North => -1,
            Direction::South => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    symbol: char,
    // Connections in North, East, South, West order.
    connections: [bool; 4],
}

impl Tile {
    const GROUND: Tile = Tile { symbol: '.', connections: [false, false, false, false] };
    const START: Tile = Tile { symbol: 'S', connections: [true, true, true, true] };
    const VERTICAL: Tile = Tile { symbol: '|', connections: [true, false, true, false] };
    const HORIZONTAL: Tile = Tile { symbol: '-', connections: [false, true, false, true] };
    const NORTH_EAST: Tile = Tile { symbol: 'L', connections: [true, true, false, false] };
    const NORTH_WEST: Tile = Tile { symbol: 'J', connections: [true, false, false, true] };
    const SOUTH_EAST: Tile = Tile { symbol: 'F', connections: [false, true, true, false] };
    const SOUTH_WEST: Tile = Tile { symbol: '7', connections: [false, false, true, true] };

    pub fn all() -> impl Iterator<Item = &'static Tile> {
        [Tile::GROUND, Tile::START, Tile::VERTICAL, Tile::HORIZONTAL, Tile::NORTH_EAST, Tile::NORTH_WEST, Tile::SOUTH_EAST, Tile::SOUTH_WEST].iter()
    }

    // Return whether the current tile connects to the other tile in the given direction.
    fn connects_with(&self, other: &Tile, direction: &Direction) -> bool {
        let self_connects_to_other = self.connections[*direction as usize];
        let other_connects_to_self = other.connections[direction.opposite() as usize];
        self_connects_to_other && other_connects_to_self
    }
}

impl std::str::FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err("string too long")
        }
        if let Some(c) = s.chars().next() {
            for tile in Tile::all() {
                if tile.symbol == c {
                    return Ok(*tile)
                }
            }
            return Err("unknown tile")
        }
        Err("empty string")
    }
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Maze {
    pub fn find_loop_length(&self) -> usize {
        let mut result = 0;
        let mut current = self.start;
        let mut previous = None;
        loop {
            //self.print_maze(Some(current));

            // Find a connected tile that is not going back
            let mut next = None;
            for (x, y, d) in self.connected_tiles(current) {
                match previous {
                    Some((px, py)) => {
                        if (x, y) != (px, py) {
                            next = Some((x, y));
                            break;
                        }
                    },
                    None => {
                        next = Some((x, y));
                        break
                    },
                }
            }
            // Walk it
            previous = Some(current);
            current = next.unwrap();
            result += 1;

            if current == self.start {
                break;
            }
        }
        result
    }

    fn connected_tiles(&self, node: (usize, usize)) -> impl Iterator<Item = (usize, usize, Direction)> {
        let (x, y) = node;
        let tile = self.tiles[y][x];

        let mut result = vec![];
        for d in [Direction::North, Direction::East, Direction::South, Direction::West].iter() {
            let (dx, dy) = (d.x_offset(), d.y_offset());
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx as usize >= self.tiles[0].len() || ny < 0 || ny as usize >= self.tiles.len() {
                continue
            }
            let other = &self.tiles[ny as usize][nx as usize];
            if tile.connects_with(other, d) {
                println!("tile {:?} connects to {:?} in {:?} direction", tile.symbol, other.symbol, d);
                result.push((nx as usize, ny as usize, *d));
            }
        }

        result.into_iter()
    }

    #[allow(dead_code)]
    fn print_maze(&self, current: Option<(usize, usize)>) {
        for (y, row) in self.tiles.iter().enumerate() {
            print!("{:04} ", y);
            for (x, tile) in row.iter().enumerate() {
                if Some((x, y)) == current {
                    print!("*");
                } else {
                    print!("{}", tile.symbol);
                }
            }
            println!();
        }
    }
}

impl std::str::FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        let mut start = None;
        for (y, line) in s.trim().lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = c.to_string().parse::<Tile>().unwrap();
                if tile == Tile::START {
                    start = Some((x, y));
                }
                row.push(tile.to_owned());
            }
            tiles.push(row);
        }
        Ok(Maze { tiles, start: start.unwrap() })
    }
}

pub fn main() {
    match std::fs::read_to_string("day10.input") {
        Ok(input) => {
            let maze = input.parse::<Maze>().unwrap();
            let loop_length = maze.find_loop_length();
            println!("loop length = {:?}, farthest distance = {:?}", loop_length, loop_length / 2);
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_connections() {
        // Start connects to tiles in all directions (assuming they connect back).
        // The cases below use Start as a "joker".
        for d in [Direction::North, Direction::South, Direction::East, Direction::West].iter() {
            assert!(Tile::START.connects_with(&Tile::START, d));
        }
        // Ground connects to nothing.
        for d in [Direction::North, Direction::South, Direction::East, Direction::West].iter() {
            assert!(!Tile::GROUND.connects_with(&Tile::START, d));
        }

        // Vertical connects to North and South, but not East and West.
        assert!(Tile::VERTICAL.connects_with(&Tile::START, &Direction::North));
        assert!(!Tile::VERTICAL.connects_with(&Tile::START, &Direction::East));
        assert!(Tile::VERTICAL.connects_with(&Tile::START, &Direction::South));
        assert!(!Tile::VERTICAL.connects_with(&Tile::START, &Direction::West));

        // Horizontal connects to East and West, but not North and South.
        assert!(!Tile::HORIZONTAL.connects_with(&Tile::START, &Direction::North));
        assert!(Tile::HORIZONTAL.connects_with(&Tile::START, &Direction::East));
        assert!(!Tile::HORIZONTAL.connects_with(&Tile::START, &Direction::South));
        assert!(Tile::HORIZONTAL.connects_with(&Tile::START, &Direction::West));

        // Special cases, non-exhaustive.
        assert!(Tile::HORIZONTAL.connects_with(&Tile::NORTH_WEST, &Direction::East), "Failed - > J");
        assert!(Tile::HORIZONTAL.connects_with(&Tile::NORTH_EAST, &Direction::West), "Failed L < -");
        assert!(Tile::HORIZONTAL.connects_with(&Tile::SOUTH_WEST, &Direction::East), "Failed - > 7");
        assert!(Tile::HORIZONTAL.connects_with(&Tile::SOUTH_EAST, &Direction::West), "Failed F < -");
    }

    #[test]
    fn tile_connections_test() {
        assert!(!Tile::VERTICAL.connects_with(&Tile::VERTICAL, &Direction::East), "Failed | > |");
        assert!(!Tile::VERTICAL.connects_with(&Tile::VERTICAL, &Direction::West), "Failed | < |");
        assert!(!Tile::HORIZONTAL.connects_with(&Tile::HORIZONTAL, &Direction::North), "Failed - ^ -");
        assert!(!Tile::HORIZONTAL.connects_with(&Tile::HORIZONTAL, &Direction::South), "Failed - v -");
    }

    #[test]
    fn part1_example1() {
        static MAZE: &str = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_loop_length(), 8);
    }

    #[test]
    fn part1_example2() {
        static MAZE: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_loop_length(), 16);
    }
}
