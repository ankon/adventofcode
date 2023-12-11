struct Chart {
    _rows: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
}

impl std::str::FromStr for Chart {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut galaxies = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                }
                row.push(c)
            }
            rows.push(row);
        }
        Ok(Chart { _rows: rows, galaxies })
    }
}

impl Chart {
    fn sum_of_shortest_paths(&self, expansion_factor: usize) -> usize {
        // Find the "expanding" rows and columns
        let mut rows_with_galaxies = Vec::new();
        let mut columns_with_galaxies = Vec::new();
        for (x, y) in self.galaxies.iter() {
            if !rows_with_galaxies.contains(y) {
                rows_with_galaxies.push(*y);
            }
            if !columns_with_galaxies.contains(x) {
                columns_with_galaxies.push(*x);
            }
        }

        let mut result = 0;
        for (g1, (x1, y1)) in self.galaxies.iter().enumerate() {
            for (g2, (x2, y2)) in self.galaxies.iter().enumerate().skip(g1 + 1) {
                // if g1 != 7 || g2 != 8 {
                //     continue;
                // }
                print!("{}({}, {}) -> {}({}, {}) = ", g1, x1, y1, g2, x2, y2);

                let mut d: usize = 0;
                let rx = if x1 <= x2 { *x1..*x2 } else { *x2..*x1 };
                let ry = if y1 <= y2 { *y1..*y2 } else { *y2..*y1 };
                for x in rx {
                    if columns_with_galaxies.contains(&x) {
                        d += 1;
                    } else {
                        d += expansion_factor;
                    }
                }
                for y in ry {
                    if rows_with_galaxies.contains(&y) {
                        d += 1;
                    } else {
                        d += expansion_factor;
                    }
                }

                println!("{}", d);
                result += d;
            }
        }
        result
    }
}

pub fn main() {
    match std::fs::read_to_string("day11.input") {
        Ok(input) => {
            if let Ok(chart) = input.parse::<Chart>() {
                println!("number of galaxies = {}", chart.galaxies.len());
                println!("sum of shortest paths = {}", chart.sum_of_shortest_paths(2));
                println!("sum of shortest paths (part 2) = {}", chart.sum_of_shortest_paths(1e6 as usize));
            } else {
                println!("cannot parse chart");
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CHART: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example1() {
        let chart = CHART.parse::<Chart>().ok().unwrap();
        assert_eq!(chart.galaxies.len(), 9);
        assert_eq!(chart.sum_of_shortest_paths(2), 374);
    }

    #[test]
    fn part2_example1() {
        let chart = CHART.parse::<Chart>().ok().unwrap();
        assert_eq!(chart.sum_of_shortest_paths(10), 1030);
        assert_eq!(chart.sum_of_shortest_paths(100), 8410);
    }
}
