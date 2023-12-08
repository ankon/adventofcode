extern crate muldiv;
use muldiv::MulDiv;
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
        // println!("{}::lookup({})", self.name, key);
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
    fn find_map(&self, from: &str) -> Option<(&str, &AlmanacMap)> {
        for m in self.maps.values() {
            if let Some((map_from, map_to)) = m.name.split_once("-to-") {
                if map_from == from {
                    return Some((map_to, m))
                }
            }
        }
        None
    }

    fn mapping<'a>(&'a self, from: &'a str, to: &'a str) -> impl Fn(usize) -> usize + 'a  {
        let mut next_from = from;
        let mut maps = vec![];

        while next_from != to {
            println!("mapping: {} -> {}", next_from, to);
            if let Some((map_to, m)) = self.find_map(next_from) {
                // This is the next map to follow
                maps.push(m);
                next_from = map_to;
            }
        }

        move |v| {
            let mut result = v;
            for m in maps.iter() {
                result = m.lookup(result)
            }
            result
        }
    }

    fn lowest_location(&self, use_ranges: bool) -> usize {
        // First we need to find a "path" from a "seed-to-" map
        // to a "-to-location" map. We can then apply the same path
        // for each of the seeds, and output the minimum location value.
        let mut result = std::usize::MAX;
        let map = self.mapping("seed", "location");
        if use_ranges {
            let mut i = 0;
            while i < self.seeds.len() {
                let start = self.seeds[i];
                let length = self.seeds[i+1];
                i += 2;

                for j in 0..length {
                    if j % 1000 == 0 {
                        print!("\rscanning range {}..{}: {}%", start, start + length, (j as u64).mul_div_floor(100, length as u64).unwrap());
                    }
                    result = min(result, map(start + j));
                }
                println!()
            }
        } else {
            for i in &self.seeds {
                result = min(result, map(*i));
            }
        }
        result
    }
}

pub fn main() {
    match std::fs::read_to_string("day5.input") {
        Ok(input) => {
            if let Ok(almanac) = input.parse::<Almanac>() {
                println!("lowest_location (part 1) = {}", almanac.lowest_location(false));
                println!("lowest_location (part 2) = {}", almanac.lowest_location(true));
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn part1_example() {
        let almanac = DATA.parse::<Almanac>().unwrap();
        assert_eq!(almanac.lowest_location(false), 35);
    }

    #[test]
    fn part2_example() {
        let almanac = DATA.parse::<Almanac>().unwrap();
        assert_eq!(almanac.lowest_location(true), 46);
    }
}
