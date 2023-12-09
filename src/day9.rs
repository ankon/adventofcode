#[derive(Debug)]
struct Series {
    data: Vec<isize>,
}

fn differences(series: &Vec<isize>) -> Vec<isize> {
    let mut result = Vec::new();
    for i in 0..series.len() - 1 {
        result.push(series[i + 1] - series[i]);
    }
    result
}

fn extrapolate_next_value(series: &Vec<isize>) -> isize {
    // Build up the difference series. When all values are 0, go back, and extrapolate.
    if all_zeros(series) {
        return 0
    }

    // Calculate the differences, and extrapolate that.
    let differences = differences(series);
    let next_value = extrapolate_next_value(&differences);
    next_value + series[series.len() - 1]
}

fn extrapolate_previous_value(series: &Vec<isize>) -> isize {
    // Build up the difference series. When all values are 0, go back, and extrapolate.
    if all_zeros(series) {
        return 0
    }

    // Calculate the differences, and extrapolate that.
    let differences = differences(series);
    let previous_value = extrapolate_previous_value(&differences);
    series[0] - previous_value
}

fn all_zeros(series: &Vec<isize>) -> bool {
    for value in series {
        if *value != 0 {
            return false
        }
    }
    true
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

fn sum_of_extrapolated_values(input: &str, extrapolate: impl Fn(&Vec<isize>) -> isize) -> isize {
    let mut result = 0;
    for line in input.lines() {
        if let Ok(series) = line.parse::<Series>() {
            result += extrapolate(&series.data);
        }
    }
    result
}

pub fn main() {
    match std::fs::read_to_string("day9.input") {
        Ok(input) => {
            println!("part1 = {}", sum_of_extrapolated_values(&input, extrapolate_next_value));
            println!("part2 = {}", sum_of_extrapolated_values(&input, extrapolate_previous_value))
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
        assert_eq!(sum_of_extrapolated_values(DATA, extrapolate_next_value), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(sum_of_extrapolated_values(DATA, extrapolate_previous_value), 2);
    }
}
