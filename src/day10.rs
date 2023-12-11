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
        let self_connects_to_other = self.connects_direction(direction);
        let other_connects_to_self = other.connects_direction(&direction.opposite());
        self_connects_to_other && other_connects_to_self
    }

    fn connects_direction(&self, direction: &Direction) -> bool {
        self.connections[*direction as usize]
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
        self.find_loop().count()
    }

    fn find_loop(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut result = vec![];
        let mut current = self.start;
        let mut previous = None;
        loop {
            //self.print_maze(Some(current));

            // Find a connected tile that is not going back
            let mut next = None;
            for (x, y, _) in self.connected_tiles(current) {
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
            result.push(current);

            if current == self.start {
                break;
            }
        }
        result.into_iter()
    }

    pub fn find_num_enclosed_tiles(&self) -> usize {
        // Step 1: Mark the loop: Sort the loop tiles by y and x coordinate, so we can quickly look up whether a tile is in the loop.
        // The start tile is a bit annoying here, replace it with what it represents by finding the two tiles that connect to it in the loop.
        let start_connected_tiles = self.connected_tiles(self.start).collect::<Vec<_>>();
        assert!(start_connected_tiles.len() == 2);
        let mut start_real_tile = None;
        for t in Tile::all() {
            if *t == Tile::START {
                continue
            }
            let (x1, y1, d1) = start_connected_tiles[0];
            let (x2, y2, d2) = start_connected_tiles[1];
            if t.connects_with(&self.tiles[y1][x1], &d1) && t.connects_with(&self.tiles[y2][x2], &d2) {
                start_real_tile = Some(t);
                break;
            }
        }
        assert!(start_real_tile.is_some());

        let mut marked_tiles = self.find_loop().collect::<Vec<_>>();
        marked_tiles.sort_by(|(x1, y1), (x2, y2)| {
            if y1 < y2 {
                return std::cmp::Ordering::Less
            }
            if y1 > y2 {
                return std::cmp::Ordering::Greater
            }
            if x1 < x2 {
                return std::cmp::Ordering::Less
            }
            if x1 > x2 {
                return std::cmp::Ordering::Greater
            }
            std::cmp::Ordering::Equal
        });
        let mut marked_tiles_iter = marked_tiles.iter().peekable();

        // Step 2: Scan over the whole maze, and track which tile is "outside" and "inside" the loop.
        //         We assume the tile at -1,y is "outside", and then by scanning line per line change our understanding
        //         of "outside": When crossing a `|` tile we are now "inside", and when crossing the next one we're back "outside."
        let mut inside_tiles = vec![];

        for (y, row) in self.tiles.iter().enumerate() {
            // NB: We could probably keep the previous value: The loop is fully enclosed in the maze, so at the end of a line
            //     we must hit "outside == true".
            let mut outside = true;
            let mut first_on_loop_tile: Option<&Tile> = None;
            for (x, mut tile) in row.iter().enumerate() {
                if *tile == Tile::START {
                    println!("replacing start tile with {:?}", start_real_tile);
                    tile = start_real_tile.unwrap();
                }
                if let Some((lx, ly)) = marked_tiles_iter.peek() {
                    if *ly == y && *lx == x {
                        // Tile is part of the loop, consume this tile.
                        marked_tiles_iter.next();
                        // See whether we cross the loop boundary:
                        // - | is trivially crossing
                        // - L-*7 is crossing (L has north, 7 has south)
                        // - F-*J is crossing (F has south, J has north)
                        // We don't need to check for horizontal tiles, because we're scanning line per line,
                        // and we do need to look at 7/J tiles to start at pattern as we're going west to east.
                        if *tile == Tile::VERTICAL {
                            // println!("crossed the loop");
                            outside = !outside;
                        } else if let Some(folt) = first_on_loop_tile {
                            // We had a previous loop tile, check whether the current tile completes or cancels the crossing.
                            // The next tile is expected to be _not_ on the loop anymore, or will start another crossing.
                            if (folt.connects_direction(&Direction::North) && tile.connects_direction(&Direction::South)) || (folt.connects_direction(&Direction::South) && tile.connects_direction(&Direction::North)) {
                                // println!("finished crossing the loop");
                                outside = !outside;
                                first_on_loop_tile = None;
                            } else if (folt.connects_direction(&Direction::North) && tile.connects_direction(&Direction::North)) || (folt.connects_direction(&Direction::South) && tile.connects_direction(&Direction::South)) {
                                // println!("canceled crossing the loop");
                                first_on_loop_tile = None;
                            } else {
                                // println!("still on the loop");
                            }
                        } else if tile.connections[Direction::East as usize] {
                            // This cannot be a `-`, as we would have already had another first_on_loop_tile.
                            // println!("started crossing the loop");
                            first_on_loop_tile = Some(tile);
                        }
                    } else {
                        // Tile is not part of the loop, count it if we're "inside" right now.
                        assert!(first_on_loop_tile.is_none(), "Must not have a pending crossing when leaving the loop");
                        if !outside {
                            inside_tiles.push((x, y));
                        }
                    }
                }
                // self.print_maze(Some((x, y)), Some(&inside_tiles));
            }
            assert!(outside);
        }
        println!("Final maze:");
        self.print_maze(None, Some(&inside_tiles));

        inside_tiles.len()
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
                // println!("tile {:?} connects to {:?} in {:?} direction", tile.symbol, other.symbol, d);
                result.push((nx as usize, ny as usize, *d));
            }
        }

        result.into_iter()
    }

    #[allow(dead_code)]
    fn print_maze(&self, current: Option<(usize, usize)>, inside_tiles: Option<&Vec<(usize, usize)>>) {
        for (y, row) in self.tiles.iter().enumerate() {
            print!("{:04} ", y);
            for (x, tile) in row.iter().enumerate() {
                if Some((x, y)) == current {
                    print!("*");
                } else if inside_tiles.is_some() && inside_tiles.unwrap().contains(&(x, y)) {
                    print!("I");
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
            println!("enclosed tiles = {:?}", maze.find_num_enclosed_tiles());
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


    #[test]
    fn part2_example1() {
        static MAZE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_num_enclosed_tiles(), 4);
    }

    #[test]
    fn part2_example2() {
        static MAZE: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_num_enclosed_tiles(), 4);
    }

    #[test]
    fn part2_example3() {
        static MAZE: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_num_enclosed_tiles(), 8);
    }

    #[test]
    fn part2_example4() {
        static MAZE: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(MAZE.parse::<Maze>().ok().unwrap().find_num_enclosed_tiles(), 10);
    }
}
