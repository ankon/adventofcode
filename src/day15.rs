fn hash(s: &str) -> u8 {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value as u8
}

struct HashMap {
    boxes: Vec<Vec<(String, u8)>>,
}

impl HashMap {
    pub fn new() -> HashMap {
        HashMap {
            boxes: vec![Vec::new(); 256],
        }
    }

    pub fn put(&mut self, key: &str, value: u8) {
        let hash = hash(key);
        let bucket = &mut self.boxes[hash as usize];
        for (k, v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        bucket.push((key.to_string(), value));
    }

    pub fn del(&mut self, key: &str) {
        let hash = hash(key);
        let bucket = &mut self.boxes[hash as usize];
        for i in 0..bucket.len() {
            if bucket[i].0 == key {
                bucket.remove(i);
                return;
            }
        }
    }
}

fn sum_hashes<'a>(s: impl std::iter::Iterator<Item = &'a str>) -> usize {
    let mut sum: usize = 0;
    for step in s {
        sum += hash(step) as usize;
    }
    sum
}

fn run_initialization_steps<'a>(s: impl std::iter::Iterator<Item = &'a str>, print: bool) -> usize {
    let mut map = HashMap::new();

    // Load the lenses into the boxes
    for step in s {
        if let Some((key, value)) = step.split_once('=') {
            map.put(key, value.parse().unwrap());
        } else if let Some(key) = step.strip_suffix('-') {
            map.del(key);
        } else {
            panic!("unexpected input");
        }
    }

    // Calculate the focusing power
    let mut power = 0;
    for (i, b) in map.boxes.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        if print {
            print!("Box {}:", i);
        }
        for (slot, (key, value)) in b.iter().enumerate() {
            if print {
                print!(" [{} {}]", key, value);
            }
            power += (i + 1) * (slot + 1) * *value as usize;
        }
        if print {
            println!();
        }
    }
    power
}

pub fn main() {
    match std::fs::read_to_string("day15.input") {
        Ok(input) => {
            let sum = sum_hashes(input.trim_end().split(','));
            println!("sum of HASHes = {}", sum);
            let focusing_power = run_initialization_steps(input.trim_end().split(','), true);
            println!("focusing power = {}", focusing_power);
        },
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52u8);

        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part1() {
        assert_eq!(sum_hashes(INPUT.split(',')), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_initialization_steps(INPUT.split(','), true), 145);
    }
}
