#[derive(Debug)]
struct Series {
    data: Vec<isize>,
}

impl Series {
    pub fn extrapolate_next_value(&self) -> isize {
        Series::extrapolate(&self.data)
    }

    fn extrapolate(series: &Vec<isize>) -> isize {
        // Build up the difference series. When all values are 0, go back, and extrapolate.
        if Series::all_zeros(&series) {
            return 0
        }

        // Calculate the differences, and extrapolate that.
        let mut differences = Vec::new();
        for i in 0..series.len() - 1 {
            differences.push(series[i + 1] - series[i]);
        }
        let next_value = Series::extrapolate(&differences);
        next_value + series[series.len() - 1]
    }

    fn all_zeros(series: &Vec<isize>) -> bool {
        for value in series {
            if *value != 0 {
                return false
            }
        }
        true
    }
}

impl std::str::FromStr for Series {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_ascii_whitespace().map(isize::from_str).collect::<Result<Vec<isize>, _>>().map_err(|_| "cannot parse series");
        match data {
            Ok(data) => Ok(Self { data }),
            Err(reason) => Err(reason),
        }
    }
}

fn sum_of_extrapolated_values(input: &str) -> isize {
    let mut result = 0;
    for line in input.lines() {
        if let Ok(series) = line.parse::<Series>() {
            result += series.extrapolate_next_value();
        }
    }
    result
}

pub fn main() {
    match std::fs::read_to_string("day9.input") {
        Ok(input) => {
            println!("part1 = {}", sum_of_extrapolated_values(&input));
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(sum_of_extrapolated_values(DATA), 114);
    }
}
