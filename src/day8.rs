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

    fn count_ghost_steps(&self, instructions: &[char]) -> Option<usize> {
        let mut steps = 0;
        let mut current = self.nodes.keys().filter(|name| name.ends_with('A')).collect::<Vec<&String>>();
        let mut ip = 0;
        while !Self::all_end_with_z(&current) {
            // Process all nodes
            for node in current.iter_mut() {
                if let Some((left, right)) = self.nodes.get(*node) {
                    let next = if instructions[ip] == 'L' { left } else { right };
                    *node = next;
                } else {
                    return None
                }
            }
            steps += 1;
            ip = (ip + 1) % instructions.len();
            if ip == 0 {
                println!("steps = {}, current = {:?}", steps, current)
            }
        }
        Some(steps)
    }

    fn all_end_with_z(nodes: &[&String]) -> bool {
        nodes.iter().all(|name| name.ends_with('Z'))
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
                println!("number of ghost steps (part 2) = {}", network.count_ghost_steps(&instructions.chars().collect::<Vec<char>>()).unwrap());
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

    #[test]
    fn part2_example1() {
        static NETWORK_DATA: &str = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(NETWORK_DATA.parse::<Network>().ok().unwrap().count_ghost_steps(&['L', 'R']), Some(6))
    }
}
