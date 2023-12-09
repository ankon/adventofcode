use std::collections::HashMap;

struct Network {
    nodes: HashMap<String, (String, String)>
}

impl Network {
    fn count_steps(&self, from: &str, to: &str, instructions: &[char]) -> Option<usize> {
        let mut steps = 0;
        let mut current = from;
        let mut ip = 0;
        while current != to {
            if let Some((left, right)) = self.nodes.get(current) {
                let next = if instructions[ip] == 'L' { left } else { right };
                current = next;
                steps += 1;
                ip = (ip + 1) % instructions.len();
            } else {
                return None
            }
        }
        Some(steps)
    }
}

impl std::str::FromStr for Network {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        for line in s.lines().filter(|line| !line.is_empty()) {
            if let Some((name, connections)) = line.split_once(" = ") {
                let connections = connections.trim_matches(|c| c == '(' || c == ')');
                if let Some((left, right)) = connections.split_once(", ") {
                    nodes.insert(name.to_string(), (left.to_string(), right.to_string()));
                } else {
                    return Err("cannot parse connections")
                }
            } else {
                println!("cannot parse node: {}", line);
                return Err("cannot parse node")
            }
        }
        Ok(Self { nodes })
    }
}

pub fn main() {
    match std::fs::read_to_string("day8.input") {
        Ok(input) => {
            // First line: Instructions
            // Second and following (non-empty) lines: Network nodes.
            if let Some((instructions, network_data)) = input.split_once('\n') {
                let network = network_data.parse::<Network>().unwrap();

                println!("number of steps from AAA to ZZZ (part 1) = {}", network.count_steps("AAA", "ZZZ", &instructions.chars().collect::<Vec<char>>()).unwrap());
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
        static NETWORK_DATA: &str = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(NETWORK_DATA.parse::<Network>().ok().unwrap().count_steps("AAA", "ZZZ", &['R', 'L']), Some(2))
    }

    #[test]
    fn part1_example2() {
        static NETWORK_DATA: &str = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(NETWORK_DATA.parse::<Network>().ok().unwrap().count_steps("AAA", "ZZZ", &['L', 'L', 'R']), Some(6))
    }
}
