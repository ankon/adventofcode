#[derive(Debug)]
struct Line {
    digits: Vec<u32>,
}

impl Line {
    fn new() -> Line {
        Line {
            digits: [].to_vec(),
        }
    }

    pub fn visit_digit(&mut self, digit: u32) {
        self.digits.push(digit);
    }

    pub fn value(&self) -> u32 {
        let first = self.digits.first().unwrap();
        let last = self.digits.last().unwrap();
        10 * first + last
    }

    fn match_digit_word_prefix(s: &str, at: usize) -> Option<(u32, usize)> {
        static DIGIT_WORDS: [&str; 9] = [ /* "zero", */ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

        let mut result = None;
        for (digit, word) in DIGIT_WORDS.iter().enumerate() {
            // println!("word = {}, word.len = {}, digit = {}, at = {}, s.len = {}", word, word.len(), digit, at, s.len());
            // println!("s.get = {:?}", s.get(at..at+word.len()).unwrap());
            if let Some(w) = s.get(at..at+word.len()) {
                // println!("w = {}, word = {}", w, *word);
                if w == *word {
                    result = Some(((digit + 1) as u32, word.len()));
                    break;
                }
            }
        }
        result
    }
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut line = Line::new();
        while i < s.len() {
            if let Some((digit, _len)) = Line::match_digit_word_prefix(s, i) {
                line.visit_digit(digit);
                // oneight...
                i += 1;
            } else {
                let sl = s.get(i..i+1).unwrap();
                // println!("sl = {}, i = {}", sl, i);
                let c = sl.chars().next().unwrap();
                if let Some(digit) = c.to_digit(10) {
                    line.visit_digit(digit);
                }
                i += 1;
            }
        }
        Ok(line)
    }
}

fn calculate_calibration_value(input: &str) -> u32 {
    let mut result = [].to_vec();
    for (index, line) in input.lines().enumerate() {
        let parsed_line = line.parse::<Line>().unwrap();
        let value = parsed_line.value();
        println!("index = {}, line = {}, parsed_line = {:?}, value = {}", index, line, parsed_line, value);
        result.push(value);
    }
    // println!("values = {:?}", result);
    result.iter().sum()
}

fn main() {
    match std::fs::read_to_string("day1.input") {
        Ok(input) => println!("value = {}", calculate_calibration_value(&input)),
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_match_digit_word_start() {
        let result = Line::match_digit_word_prefix("two29eighteight1", 0);
        assert_eq!(result, Some((2, 3)));
    }

    #[test]
    fn test_match_digit_word_middle() {
        let result = Line::match_digit_word_prefix("two29eighteight1", 5);
        assert_eq!(result, Some((8, 5)));
    }

    #[test]
    fn calculate_calibration_value_example1() {
        static DATA: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let value = calculate_calibration_value(DATA);
        assert_eq!(142, value);
    }

    #[test]
    fn calculate_calibration_value_example2() {
        static DATA: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let value = calculate_calibration_value(DATA);
        assert_eq!(281, value);
    }

    #[test]
    fn calculate_calibration_value_oneight() {
        let result = calculate_calibration_value("oneight");
        assert_eq!(result, 18);
    }
}
