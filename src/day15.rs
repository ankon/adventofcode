fn hash(s: &str) -> u8 {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value as u8
}

fn sum_hashes<'a>(s: impl std::iter::Iterator<Item = &'a str>) -> usize {
    let mut sum: usize = 0;
    for step in s {
        sum += hash(step) as usize;
    }
    sum
}

pub fn main() {
    match std::fs::read_to_string("day15.input") {
        Ok(input) => {
            let sum = sum_hashes(input.trim_end().split(','));
            println!("sum of HASHes = {}", sum);
        },
        Err(reason) => println!("error = {}", reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(sum_hashes(input.split(',')), 1320);
    }
}
