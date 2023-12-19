use core::panic;
use std::vec;

#[derive(Debug)]
struct Pattern {
    // The pattern is a 2D array of characters, where each character is either a '#' or a '.'.
    pattern: Vec<Vec<char>>,

    // Bit pattern per column, # is 1.
    // The MSB represents the top row, the LSB the bottom row (possibly filled up with 0).
    columns: Vec<u32>,
    // Bit pattern per row, # is 1.
    // The MSB represents the left column, the LSB the right column (possibly filled up with 0).
    rows: Vec<u32>,
}

impl Pattern {
    fn find_mirror_column(&self, errors: u32, print: bool) -> impl Iterator<Item = usize> {
        let mut result = vec![];
        let mut last = None;
        for (i, c) in self.columns.iter().enumerate() {
            if print {
                println!("find_mirror_column: i = {}, c = {:032b}, last = {:032b}", i, c, last.unwrap_or(0));
            }
            if let Some(l) = last {
                // Calculate the number of errors between this and the previous row. If we still have some
                // left from our budget, proceed checking further.
                let mut errors_left  = (errors as i32) - (((l as u32) ^ *c).count_ones() as i32);
                if errors_left >= 0 {
                    if print {
                        println!("find_mirror_column: found potential mirror line at i = {}", i);
                    }

                    // Found a potential mirror line
                    // Iterate outwards from here
                    let mut found = true;
                    for d in 1.. {
                        let check_column = i + d;
                        let other_column: isize = (i as isize) - (d as isize) - 1;
                        if check_column >= self.columns.len() || other_column < 0 {
                            if print {
                                println!("find_mirror_column: hit boundary, i = {}, d = {}, check_column = {}, other_column = {}", i, d, check_column, other_column);
                            }
                            break;
                        }
                        // Count errors between these rows.
                        let errors = (self.columns[check_column] ^ self.columns[other_column as usize]).count_ones() as i32;
                        errors_left -= errors;
                        if errors_left < 0 {
                            if print {
                                println!("find_mirror_column: not matching, i = {}, d = {}, [{}] = {:032b} <> [{}] = {:032b}", i, d, check_column, self.columns[check_column], other_column, self.columns[other_column as usize]);
                            }
                            found = false;
                            break;
                        }

                        // Proceed with the next pair.
                    }

                    if found && errors_left == 0 {
                        result.push(i);
                    }
                }
            }
            last = Some(*c);
        }

        result.into_iter()
    }

    fn find_mirror_row(&self, errors: u32, print: bool) -> impl Iterator<Item = usize> {
        let mut result = vec![];
        let mut last = None;
        for (i, c) in self.rows.iter().enumerate() {
            if print {
                println!("find_mirror_row: row = \"{}\", i = {}, c = {:032b}, last = {:032b}", self.pattern[i].iter().collect::<String>(), i, c, last.unwrap_or(0));
            }
            if let Some(l) = last {
                // Calculate the number of errors between this and the previous row. If we still have some
                // left from our budget, proceed checking further.
                let mut errors_left  = (errors as i32) - (((l as u32) ^ *c).count_ones() as i32);
                if errors_left >= 0 {
                    // Found a potential mirror line
                    // Iterate outwards from here
                    if print {
                        println!("find_mirror_row: found potential mirror line at i = {}", i);
                    }

                    let mut found = true;
                    for d in 1.. {
                        let check_row = i + d;
                        let other_row: isize = (i as isize) - (d as isize) - 1;
                        if check_row >= self.pattern.len() || other_row < 0 {
                            if print {
                                println!("find_mirror_row: hit boundary, i = {}, d = {}, check_row = {}, other_row = {}", i, d, check_row, other_row);
                            }
                            break;
                        }
                        // Count errors between these rows.
                        let errors = (self.rows[check_row] ^ self.rows[other_row as usize]).count_ones() as i32;
                        errors_left -= errors;
                        if errors_left < 0 {
                            if print {
                                println!("find_mirror_row: not matching, i = {}, d = {}, [{}] = {:032b} <> [{}] = {:032b}", i, d, check_row, self.rows[check_row], other_row, self.rows[other_row as usize]);
                            }
                            found = false;
                            break;
                        }

                        // Proceed with the next pair.
                    }

                    if found && errors_left == 0 {
                        result.push(i);
                    }
                }
            }
            last = Some(*c);
        }

        result.into_iter()
    }

    pub fn find_mirror(&self, max_errors: u32, print: bool) -> impl Iterator<Item = (Option<usize>, Option<usize>)> {
        // Search until we find the same column twice: If it is a mirror line, then we can extend from there
        // and compare the columns. If both comparison directions hit the border, the line is a mirror, otherwise
        // proceed.
        // NB: The task doesn't specify whether horizontal or vertical is "more important", so in theory both
        //     configurations should give the same results ...
        self.find_mirror_row(max_errors, print).map(|row| (None, Some(row))).chain(
            self.find_mirror_column(max_errors, print).map(|column| (Some(column), None))
        )
        // self.find_mirror_column(max_errors, print).map(|column| (Some(column), None)).chain(
        //     self.find_mirror_row(max_errors, print).map(|row| (None, Some(row)))
        // )
    }
}

impl std::str::FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = vec![];
        let mut rows = vec![];
        let pattern = s.lines().enumerate().map(|(r, line)| {
            let pattern_row: Vec<_> = line.chars().collect();
            let mut row = 0;
            for (c, ch) in pattern_row.iter().enumerate() {
                if c >= columns.len() {
                    columns.push(0);
                }

                // Note that we don't know the width or height here, but to get the bits
                // into a natural order we can just align them on the word size instead:
                //
                //   ##...##..
                //   123456789
                // 1 110001100
                if *ch == '#' {
                    columns[c] |= 1 << (31 - r);
                    row |= 1 << (31 - c);
                }
            }
            rows.push(row);
            pattern_row
        }).collect();
        Ok(Pattern {
            pattern,
            columns,
            rows,
        })
    }
}

fn print_pattern(pattern: &Pattern, prefix: impl Fn(usize) -> String, suffix: impl Fn(usize) -> String) {
    for (r, row) in pattern.pattern.iter().enumerate() {
        print!("{}", prefix(r));
        for c in row.iter() {
            print!("{}", c);
        }
        println!("{}", suffix(r));
    }
}

fn print_pattern_with_column_indicator(pattern: &Pattern, column: usize) {
    let row_len = pattern.pattern[0].len();
    let mut header = String::from("");
    for i in 0..row_len {
        header.push((((i + 1) % 10) as u8 + b'0') as char);
    }
    let mut indicator = String::from("");
    for i in 0..row_len {
        if i == column - 1 {
            indicator.push('>');
        } else if i == column {
            indicator.push('<');
        } else {
            indicator.push(' ');
        }
    }

    println!("{}", header);
    println!("{}", indicator);
    print_pattern(pattern, |_| String::from(""), |_| String::from(""));
    println!("{}", indicator);
    println!("{}", header);
}

fn print_pattern_with_row_indicator(pattern: &Pattern, row: usize) {
    print_pattern(pattern, |r| {
        let mut prefix = String::from((((r + 1) % 10) as u8 + b'0') as char);
        if r == row - 1 {
            prefix.push('v');
        } else if r == row {
            prefix.push('^');
        } else {
            prefix.push(' ');
        }
        prefix
    }, |r| {
        let mut prefix = String::from("");
        if r == row - 1 {
            prefix.push('v');
        } else if r == row {
            prefix.push('^');
        } else {
            prefix.push(' ');
        }
        prefix.push((((r + 1) % 10) as u8 + b'0') as char);
        prefix
    })
}

fn print_pattern_and_mirror_indicators(pattern: &Pattern, column: Option<usize>, row: Option<usize>) {
    if let Some(column) = column {
        print_pattern_with_column_indicator(pattern, column);
    } else if let Some(row) = row {
        print_pattern_with_row_indicator(pattern, row);
    } else {
        print_pattern(pattern, |_| String::from(""), |_| String::from(""));
    }
}

pub fn main() {
    // Part 1: 0, Part 2: 1.
    static ERRORS: u32 = 1;

    match std::fs::read_to_string("day13.input") {
        Ok(mut input) => {
            let mut result = 0;
            let mut tmp = String::from("");
            // Make sure we can find an empty line at the end:
            input.push('\n');
            for (i, line) in input.split('\n').enumerate() {
                if line.is_empty() {
                    // Ignore multiple consecutive empty lines
                    if tmp.is_empty() {
                        continue;
                    }
                    let pattern = tmp.parse::<Pattern>().unwrap();
                    let mut num_mirror_lines = 0;
                    for (c, r) in pattern.find_mirror(ERRORS, false) {
                        if num_mirror_lines == 0 {
                            print_pattern_and_mirror_indicators(&pattern, c, r);
                        }
                        if let Some(columns_before) = c {
                            println!("columns_before = {}", columns_before);
                            num_mirror_lines += 1;
                            result += columns_before;
                        } else if let Some(rows_before) = r {
                            println!("rows_before = {}", rows_before);
                            num_mirror_lines += 1;
                            result += 100 * rows_before;
                        } else {
                            panic!("no mirror found, line = {}, pattern = {:?}", i, pattern);
                        }
                        if num_mirror_lines == 1 {
                            break;
                        }
                    }
                    println!();
                    tmp.clear();
                } else {
                    tmp.push_str(line);
                    tmp.push('\n');
                }
            }
            println!("summarized with {} errors: {}", ERRORS, result);
        }
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    static DATA2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn example1_data1() {
        let pattern = DATA1.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(0, true).collect::<Vec<_>>(), vec![(Some(5), None)]);
        print_pattern_with_column_indicator(&pattern, 5)
    }

    #[test]
    fn example1_data2() {
        let pattern = DATA2.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(0, true).collect::<Vec<_>>(), vec![(None, Some(4))]);
        print_pattern_with_row_indicator(&pattern, 4)
    }

    #[test]
    fn example1_test1() {
        const INPUT: &str = "##...##..
.##.#..#.
.#.......
...#.##.#
##..#..#.
.##..##..
.#...##..
#.#######
....####.
..##.##.#
..##.##.#
....####.
#.#######
.#...##..
.##..##..
#...#..#.
...#.##.#";
        let pattern = INPUT.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(0, true).collect::<Vec<_>>(), vec![(Some(6), None)]);
    }

    #[test]
    fn example1_test2() {
        const INPUT: &str = "....#....##....
.##..##.####.##
#..#.#...##...#
.##.####....###
#..#.##.####.##
#..###.#.##.#.#
.##.##.##..####";
        let pattern = INPUT.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(0, true).collect::<Vec<_>>(), vec![(Some(2), None)]);
    }

    #[test]
    fn example2_data1() {
        let pattern = DATA1.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(1, true).collect::<Vec<_>>(), vec![(None, Some(3))]);
        print_pattern_with_row_indicator(&pattern, 3)
    }

    #[test]
    fn example2_data2() {
        let pattern = DATA2.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(1, true).collect::<Vec<_>>(), vec![(None, Some(1))]);
        print_pattern_with_row_indicator(&pattern, 1)
    }
}
