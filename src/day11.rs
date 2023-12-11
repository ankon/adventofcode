struct Chart {
    rows: Vec<Vec<char>>,
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
        Ok(Chart { rows, galaxies })
    }
}

impl Chart {
    fn sum_of_shortest_paths(&self) -> usize {
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
                print!("{}({}, {}) -> {}({}, {}) = ", g1, x1, y1, g2, x2, y2);

                let mut rx = *x1+1..*x2;
                let mut dx = (*x2 as isize) - (*x1 as isize);
                if dx < 0 {
                    dx = -dx;
                    rx = x2+1..*x1;
                }
                let mut ry = *y1+1..*y2;
                let mut dy = (*y2 as isize) - (*y1 as isize);
                if dy < 0 {
                    dy = -dy;
                    ry = y2+1..*y1;
                }
                for x in rx {
                    if !columns_with_galaxies.contains(&x) {
                        dx += 1;
                    }
                }
                for y in ry {
                    if !rows_with_galaxies.contains(&y) {
                        dy += 1;
                    }
                }

                println!("{}", dx + dy);
                result += (dx + dy) as usize;
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
                println!("sum of shortest paths = {}", chart.sum_of_shortest_paths());
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

    #[test]
    fn part1_example1() {
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
        let chart = CHART.parse::<Chart>().ok().unwrap();
        assert_eq!(chart.galaxies.len(), 9);
        assert_eq!(chart.sum_of_shortest_paths(), 374);
    }
}
