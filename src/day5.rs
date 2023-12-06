use std::{collections::HashMap, cmp::min};

#[derive(Debug)]
struct AlmanacRange {
    destination_min: usize,
    source_min: usize,
    length: usize,
}

impl AlmanacRange {
    fn new(destination_min: usize, source_min: usize, length: usize) -> Self {
        AlmanacRange { destination_min, source_min, length }
    }

    fn lookup(&self, key: usize) -> Option<usize> {
        if key < self.source_min {
            return None;
        }
        if key >= self.source_min + self.length {
            return None;
        }

        let result = self.destination_min + (key - self.source_min);
        Some(result)
    }
}

/// A mapping in the almanac.
#[derive(Debug)]
struct AlmanacMap {
    name: String,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn new(name: &str) -> Self {
        AlmanacMap { name: String::from(name), ranges: vec![] }
    }

    fn append_range(&mut self, destination_min: usize, source_min: usize, length: usize) {
        self.ranges.push(AlmanacRange::new(destination_min, source_min, length));
    }

    fn lookup(&self, key: usize) -> usize {
        println!("{}::lookup({})", self.name, key);
        for range in &self.ranges {
            if let Some(mapped) = range.lookup(key) {
                return mapped;
            }
        }
        key
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, AlmanacMap>
}

impl std::str::FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        // First line: seeds.
        let seeds_line = lines[0].split_once(':');
        if seeds_line.is_none() {
            return Err("invalid line: no delimiter");
        } else if seeds_line.unwrap().0 != "seeds" {
            return Err("invalid line: no seeds keyword");
        }
        println!("parsing seeds {}", seeds_line.unwrap().1);
        let seeds = seeds_line.unwrap().1
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<usize>>();

        // Parse the remaining lines as maps: A title line identifying the map name (`X-to-Y map:`), followed
        // by 1 or more lines of the form `destination_min source_min length`.
        let mut maps = HashMap::<String, AlmanacMap>::new();
        let mut map: Option<AlmanacMap> = None;
        for line in &lines[1..] {
            if line.trim().is_empty() {
                if let Some(m) = map.take() {
                    maps.insert(m.name.clone(), m);
                }
                continue
            }

            if map.is_some() {
                let parts = line.split(' ').collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err("invalid map line");
                }
                let destination_min = parts[0].parse::<usize>().unwrap();
                let source_min = parts[1].parse::<usize>().unwrap();
                let length = parts[2].parse::<usize>().unwrap();
                map.as_mut().unwrap().append_range( destination_min, source_min, length);
            } else if let Some((name, map_keyword)) = line.split_once(' ') {
                if map_keyword != "map:" {
                    return Err("invalid line: no map keyword");
                }
                map = Some(AlmanacMap::new(name));
            } else {
                return Err("invalid line: expected a map");
            }
        }
        if let Some(m) = map.take() {
            maps.insert(m.name.clone(), m);
        }
        Ok(Almanac { seeds, maps })
    }
}

impl Almanac {
    fn mapping(&self, from: &str, to: &str) -> Option<Box<dyn Fn(usize) -> usize + '_>> {
        if from == to {
            return Some(Box::new(|v| v))
        }

        println!("mapping({}, {})?", from, to);
        for m in self.maps.values() {
            if let Some((map_from, map_to)) = m.name.split_once("-to-") {
                if map_from != from {
                    continue;
                }
                if let Some(result) = self.mapping(map_to, to) {
                    let composed = move |v| result(m.lookup(v));
                    return Some(Box::new(composed))
                }
            }
        }
        println!("[x] not found");
        None
    }

    fn lowest_location(&self) -> usize {
        // First we need to find a "path" from a "seed-to-" map
        // to a "-to-location" map. We can then apply the same path
        // for each of the seeds, and output the minimum location value.
        let mut result = std::usize::MAX;
        if let Some(map) =self.mapping("seed", "location") {
            for i in &self.seeds {
                result = min(result, map(*i));
            }
            return result
        }

        panic!("Cannot find a path")
    }
}

pub fn main() {
    match std::fs::read_to_string("day5.input") {
        Ok(input) => {
            if let Ok(almanac) = input.parse::<Almanac>() {
                println!("lowest_location = {}", almanac.lowest_location());
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        static DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let almanac = DATA.parse::<Almanac>().unwrap();
        assert_eq!(almanac.lowest_location(), 35);
    }
}
