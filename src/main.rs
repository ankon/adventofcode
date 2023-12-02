const TEST_STR: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[derive(Debug)]
struct Line {
    digits: Vec<i32>,
}

impl Line {
    fn new() -> Line {
        Line {
            digits: [].to_vec(),
        }
    }

    pub fn visit_digit(&mut self, digit: i32) {
        if self.digits.len() == 2 {
            self.digits.pop();
        }
        self.digits.push(digit);
    }

    pub fn value(&self) -> i32 {
        let first = self.digits.first().unwrap();
        if let Some(last) = self.digits.get(1) {
            10 * first + last
        } else {
            10 * first + first
        }
    }
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = Line::new();
        for c in s.chars() {
            if let '0'..='9' = c {
                let digit = c.to_digit(10).unwrap();
                line.visit_digit(digit as i32);
            }
        }
        Ok(line)
    }
}

fn calculate_calibration_value(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let parsed_line = line.parse::<Line>().unwrap();
        println!("parsed_line = {:?}", parsed_line);
        result += parsed_line.value();
    }
    result
}

fn main() {
    println!("test value = {}", calculate_calibration_value(TEST_STR));
    match std::fs::read_to_string("day1.input") {
        Ok(input) => println!("real value = {}", calculate_calibration_value(&input)),
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_line_single_digit() {
        let mut line = Line::new();
        line.visit_digit(5);
        assert_eq!(line.value(), 55);
    }

    #[test]
    fn test_line_multiple_digit() {
        let mut line = Line::new();
        line.visit_digit(5);
        line.visit_digit(3);
        assert_eq!(line.value(), 53);
    }
}
