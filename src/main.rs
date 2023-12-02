const TEST_STR: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[derive(Debug)]
struct Line {
    first: Option<i32>,
    last: Option<i32>,
}

impl Line {
    fn new() -> Line {
        Line {
            first: None,
            last: None,
        }
    }

    fn visit_digit(&mut self, digit: i32) {
        if self.first.is_none() {
            self.first = Some(digit);
        } else {
            self.last = Some(digit);
        }
    }

    fn value(&self) -> i32 {
        let first = self.first.unwrap();
        if let Some(last) = self.last {
            10 * last + first
        } else {
            10 * first + first
        }
    }
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = Line::new();
        for c in s.chars().rev() {
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
}
