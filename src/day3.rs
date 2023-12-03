use std::str::from_utf8;

struct Schematic {
    width: usize,
    lines: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
struct Pos {
    y: usize,
    x: usize,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    pos: Pos,
    symbol: u8,
}

type PartNumber = u32;

impl std::str::FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<u8>> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        let width = lines[0].len();
        for line in &lines {
            if line.len() != width {
                return Err(());
            }
        }
        Ok(Schematic { width, lines })
    }
}

impl Schematic {
    fn to_u32(s: &[u8]) -> u32 {
        if let Ok(s) = from_utf8(s) {
            println!("s = {}", s);
            return s.parse::<u32>().unwrap();
        }
        panic!("invalid input")
    }

    fn symbols_around(&self, y: usize, x1: usize, l: usize) -> Vec<Symbol> {
        let mut symbols = vec![];
        for y in y.saturating_sub(1)..=y+1 {
            if y >= self.lines.len() {
                continue;
            }
            let line = &self.lines[y];
            let skip = x1.saturating_sub(1);
            for (x, c) in line.iter().enumerate().skip(skip).take(1 + l + 1) {
                print!("{}", *c as char);
                match c {
                    b'.' => continue,
                    b'0'..=b'9' => continue,
                    _ => {
                        symbols.push(Symbol { pos: Pos { y, x }, symbol: *c });
                    }
                }
            }
            println!()
        }
        println!("symbols_around({},{},{}) = {:?}", y, x1, l, symbols);
        symbols
    }

    fn sum_of_part_numbers(&self) -> u32 {
        let mut sum = 0;
        for y in 0..self.lines.len() {
            println!("y = {}", y);
            let line = &self.lines[y];
            let mut x = 0;
            while x < self.width {
                match line[x] {
                    b'.' => { x += 1; continue },
                    b'0'..=b'9' => {
                        // Part number
                        println!("found part number at ({}, {})", y, x);
                    },
                    _ => { x += 1; continue },
                }

                let mut l = 0;
                while x+l < self.width && line[x+l] >= b'0' && line[x+l] <= b'9' {
                    l += 1;
                }

                // It's a part number, check whether it is connected to a symbol
                // or not.
                if !self.symbols_around(y, x, l).is_empty() {
                    sum += Schematic::to_u32(&line[x..x+l]);
                }
                x += l;
            }
        }
        sum
    }

    fn sum_of_gear_ratios(&self) -> u32 {
        // Gears that might be connected to parts
        let mut gears: Vec<(Pos, Vec<PartNumber>)> = vec![];

        for y in 0..self.lines.len() {
            println!("y = {}", y);
            let line = &self.lines[y];
            let mut x = 0;
            while x < self.width {
                match line[x] {
                    b'.' => { x += 1; continue },
                    b'0'..=b'9' => {
                        // Part number
                        println!("found part number at ({}, {})", y, x);
                    },
                    _ => { x += 1; continue },
                }

                let mut l = 0;
                while x+l < self.width && line[x+l] >= b'0' && line[x+l] <= b'9' {
                    l += 1;
                }

                // It's a part number, check whether it is connected to a symbol
                // or not.
                for symbol in self.symbols_around(y, x, l) {
                    if symbol.symbol != b'*' {
                        continue
                    }
                    let part_number = Schematic::to_u32(&line[x..x+l]);
                    let open_gear = gears.iter_mut().find(|(gear, _)| gear.x == symbol.pos.x && gear.y == symbol.pos.y);
                    if let Some((_, parts)) = open_gear {
                        parts.push(part_number);
                    } else {
                        gears.push((symbol.pos, vec![part_number]));
                    }
                }
                x += l;
            }
        }

        println!("gears = {:?}", gears);

        // Calculate the sum of the gear ratios where each gear is connected to
        // exactly two parts.
        let mut sum = 0;
        for (_, parts) in gears {
            if parts.len() == 2 {
                let ratio = parts[0] * parts[1];
                sum += ratio;
            }
        }
        sum
    }

}

pub fn main() {
    match std::fs::read_to_string("day3.input") {
        Ok(input) => {
            if let Ok(schematic) = input.parse::<Schematic>() {
                println!("sum_of_part_numbers = {}", schematic.sum_of_part_numbers());
                println!("sum_of_gear_ratios = {}", schematic.sum_of_gear_ratios());
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_around_trivial_false() {
        let schematic = Schematic { width: 3, lines: vec!["123".as_bytes().to_vec()] };
        assert_eq!(schematic.symbols_around(0, 0, 3), vec![]);
    }

    #[test]
    fn symbols_around_just_before() {
        let schematic = Schematic { width: 4, lines: vec!["*123".as_bytes().to_vec()] };
        assert_eq!(schematic.symbols_around(0, 1, 3), vec![ Symbol { pos: Pos { y: 0 , x: 0 }, symbol: b'*' } ]);
    }

    #[test]
    fn sum_of_part_numbers_example() {
        static DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let schematic = DATA.parse::<Schematic>().unwrap();
        assert_eq!(schematic.sum_of_part_numbers(), 4361);
    }

    #[test]
    fn sum_of_gear_ratios_reduced_example() {
        static DATA: &str = "467..114..
...*......
..35..633.";
        let schematic = DATA.parse::<Schematic>().unwrap();
        assert_eq!(schematic.sum_of_gear_ratios(), 467 * 35);
    }

    #[test]
    fn sum_of_gear_ratios_example() {
        static DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let schematic = DATA.parse::<Schematic>().unwrap();
        assert_eq!(schematic.sum_of_gear_ratios(), 467835);
    }
}
