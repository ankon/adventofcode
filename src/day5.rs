use std::collections::HashMap;

#[derive(Debug)]
struct AlmanacRange {
    source_min: usize,
    destination_min: usize,
    length: usize,
}

impl AlmanacRange {
    fn new(source_min: usize, destination_min: usize, length: usize) -> Self {
        AlmanacRange { source_min, destination_min, length }
    }

    fn lookup(&self, key: usize) -> Option<usize> {
        if key < self.source_min {
            return None;
        }
        if key >= self.source_min + self.length {
            return None;
        }
        Some(self.destination_min + (key - self.source_min))
    }
}

/// A mapping in the almanac.
#[derive(Debug)]
struct AlmanacMap<'a> {
    name: &'a str,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap<'_> {
    fn append_range(&mut self, destination_min: usize, source_min: usize, length: usize) {
        self.ranges.push(AlmanacRange::new(source_min, destination_min, length));
    }

    fn lookup(&self, key: usize) -> usize {
        for range in &self.ranges {
            if let Some(mapped) = range.lookup(key) {
                return mapped;
            }
        }
        panic!("key {} not found", key);
    }
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<usize>,
    maps: HashMap<&'a str, &'a AlmanacMap<'a>>
}

impl std::str::FromStr for Almanac<'_> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        // First line: seeds.
        let seeds_line = lines[0].split_once(":");
        if seeds_line.is_none() {
            return Err("invalid line: no delimiter");
        } else if seeds_line.unwrap().0 != "seeds" {
            return Err("invalid line: no seeds keyword");
        }
        let seeds = seeds_line.unwrap().1.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        // Parse the remaining lines as maps: A title line identifying the map name (`X-to-Y map:`), followed
        // by 1 or more lines of the form `destination_min source_min length`.
        let mut maps = HashMap::<&str, &AlmanacMap>::new();
        let mut map: Option<&mut AlmanacMap<'_>> = None;
        for line in &lines[1..] {
            if line.trim().is_empty() {
                if let Some(map) = map.take() {
                    maps.insert(map.name, map);
                }
            } else if map.is_some() {
                let parts = line.split(" ").collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err("invalid map line");
                }
                let destination_min = parts[0].parse::<usize>().unwrap();
                let source_min = parts[1].parse::<usize>().unwrap();
                let length = parts[2].parse::<usize>().unwrap();
                map..append_range(source_min, destination_min, length);
            } else if let Some((name, map_keyword)) = line.split_once(" ") {
                if map_keyword != "map:" {
                    return Err("invalid line: no map keyword");
                }
                map.insert(&mut AlmanacMap { name, ranges: vec![] });
                continue;
            } else {
                return Err("invalid line: expected a map");
            }
        }
        Ok(Almanac { seeds, maps })
    }
}

impl Almanac<'_> {
    fn lowest_location(&self) -> u32 {
        unimplemented!()
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
