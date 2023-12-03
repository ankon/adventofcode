use std::str::from_utf8;

struct Schematic {
    width: usize,
    lines: Vec<Vec<u8>>,
}

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

    fn has_symbol_around(&self, y: usize, x1: usize, l: usize) -> bool {
        let mut has_symbol = false;
        'outer: for y in y.saturating_sub(1)..=y+1 {
            if y >= self.lines.len() {
                continue;
            }
            println!("has_symbol_around(y = {}, x1 = {}, l = {}): y = {}", y, x1, l, y);
            let line = &self.lines[y];
            for c in line.iter().skip(x1.saturating_sub(1)).take(1 + l + 1) {
                print!("{}", *c as char);
                match c {
                    b'.' => continue,
                    b'0'..=b'9' => continue,
                    _ => { has_symbol = true; break 'outer; }
                }
            }
            println!()
        }
        println!("has_symbol_around = {}", has_symbol);
        has_symbol
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
                if self.has_symbol_around(y, x, l) {
                    sum += Schematic::to_u32(&line[x..x+l]);
                }
                x += l;
            }
        }
        sum
    }
}

fn sum_of_part_numbers(input: &str) -> u32 {
    if let Ok(schematic) = input.parse::<Schematic>() {
        return schematic.sum_of_part_numbers();
    }
    panic!("invalid input")
}

pub fn main() {
    match std::fs::read_to_string("day3.input") {
        Ok(input) => println!("sum_of_part_numbers = {}", sum_of_part_numbers(&input)),
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_symbol_around_trivial_false() {
        let schematic = Schematic { width: 3, lines: vec!["123".as_bytes().to_vec()] };
        assert!(!schematic.has_symbol_around(0, 0, 3));
    }

    #[test]
    fn has_symbol_around_just_before() {
        let schematic = Schematic { width: 4, lines: vec!["*123".as_bytes().to_vec()] };
        assert!(schematic.has_symbol_around(0, 1, 3));
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
        assert_eq!(sum_of_part_numbers(DATA), 4361);
    }
}
