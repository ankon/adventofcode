use core::panic;
use std::vec;

#[derive(Debug)]
struct Pattern {
    // The pattern is a 2D array of characters, where each character is either a '#' or a '.'.
    pattern: Vec<Vec<char>>,

    // Count of # in each column
    columns: Vec<usize>,
    // Count of # in each row
    rows: Vec<usize>,
}

impl Pattern {
    fn find_mirror_column(&self, print: bool) -> impl Iterator<Item = usize> {
        let mut result = vec![];
        let mut last = None;
        for (i, c) in self.columns.iter().enumerate() {
            if print {
                println!("find_mirror_column: i = {}, c = {}, last = {:?}", i, c, last);
            }
            if let Some(l) = last {
                if l == *c {
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
                        // Cheap compare first: If the numbers don't match, don't bother.
                        if self.columns[check_column] != self.columns[other_column as usize] {
                            if print {
                                println!("find_mirror_column: not matching numbers, i = {}, d = {}, [{}] = {} <> [{}] = {}", i, d, check_column, self.columns[check_column], other_column, self.columns[other_column as usize]);
                            }
                            found = false;
                            break;
                        }
                        // Ok, need the patterns.
                        for row in 0..self.pattern.len() {
                            if self.pattern[row][check_column] != self.pattern[row][other_column as usize] {
                                if print {
                                    println!("find_mirror_column: not matching pattern, i = {}, d = {}, row = {}", i, d, row);
                                }
                                found = false;
                                break;
                            }
                        }

                        // Proceed with the next pair.
                    }

                    if found {
                        result.push(i);
                    }
                }
            }
            last = Some(*c);
        }

        result.into_iter()
    }

    fn find_mirror_row(&self, print: bool) -> impl Iterator<Item = usize> {
        let mut result = vec![];
        let mut last = None;
        for (i, c) in self.rows.iter().enumerate() {
            if print {
                println!("find_mirror_row: i = {}, c = {}, last = {:?}", i, c, last);
            }
            if let Some(l) = last {
                if l == *c {
                    if print {
                        println!("find_mirror_row: found potential mirror line at i = {}", i);
                    }
                    // Found a potential mirror line
                    // Iterate outwards from here
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
                        // Cheap compare first: If the numbers don't match, don't bother.
                        if self.rows[check_row] != self.rows[other_row as usize] {
                            if print {
                                println!("find_mirror_row: not matching numbers, i = {}, d = {}, [{}] = {} <> [{}] = {}", i, d, check_row, self.rows[check_row], other_row, self.rows[other_row as usize]);
                            }
                            found = false;
                            break;
                        }
                        // Ok, need the patterns.
                        for row in 0..self.pattern.len() {
                            if self.pattern[check_row] != self.pattern[other_row as usize] {
                                if print {
                                    println!("find_mirror_row: not matching pattern, i = {}, d = {}, row = {}", i, d, row);
                                }
                                found = false;
                                break;
                            }
                        }

                        // Proceed with the next pair.
                    }

                    if found {
                        result.push(i);
                    }
                }
            }
            last = Some(*c);
        }

        result.into_iter()
    }

    pub fn find_mirror(&self, print: bool) -> impl Iterator<Item = (Option<usize>, Option<usize>)> {
        // Search until we find the same column twice: If it is a mirror line, then we can extend from there
        // and compare the columns. If both comparison directions hit the border, the line is a mirror, otherwise
        // proceed.
        self.find_mirror_column(print).map(|column| (Some(column), None)).into_iter().chain(
            self.find_mirror_row(print).map(|row| (None, Some(row))).into_iter()
        )
    }
}

impl std::str::FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = vec![];
        let mut rows = vec![];
        let pattern = s.lines().map(|line| {
            let row: Vec<_> = line.chars().collect();
            let mut row_count = 0;
            for (i, c) in row.iter().enumerate() {
                if i >= columns.len() {
                    columns.push(0);
                }
                if *c == '#' {
                    columns[i] += 1;
                    row_count += 1;
                }
            }
            rows.push(row_count);
            row
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
                    for (c, r) in pattern.find_mirror(false) {
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
            println!("summarized = {}", result);
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
        assert_eq!(pattern.find_mirror(true).collect::<Vec<_>>(), vec![(Some(5), None)]);
        print_pattern_with_column_indicator(&pattern, 5)
    }

    #[test]
    fn example1_data2() {
        let pattern = DATA2.parse::<Pattern>().unwrap();
        assert_eq!(pattern.find_mirror(true).collect::<Vec<_>>(), vec![(None, Some(4))]);
        print_pattern_with_row_indicator(&pattern, 4)
    }
}
