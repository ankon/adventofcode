use std::{str::FromStr, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Condition {
    Operational = '.' as isize,
    Damaged = '#' as isize,
    Unknown = '?' as isize,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("unexpected input character"),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(s) = std::str::from_utf8(&[*self as u8]) {
            f.write_str(s)
        } else {
            panic!("I don't understand the type it wants")
        }
    }
}

struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_spring_groups: Vec<usize>,
}

impl ConditionRecord {
    #[cfg(test)]
    pub fn new(conditions: &[Condition], damaged_spring_groups: &[usize]) -> ConditionRecord {
        ConditionRecord {
            conditions: conditions.to_vec(),
            damaged_spring_groups: damaged_spring_groups.to_vec(),
        }
    }

    pub fn num_arrangements(&self) -> usize {
        // Stupid logic: Each '?' can be either '.' or '#'. So, try both, and then proceed
        // and see if _at the end_. Essentially we're building a matching automaton here?????!?!?!!
        // Each state is the conditions we found.
        let states: &mut Vec<Vec<Condition>> = &mut vec![];

        // Start out with one empty state:
        let initial_state = vec![];
        states.push(initial_state);
        for c in &self.conditions {
            // println!("{:?} with states {}", c, states.iter().map(|s| Self::format_state(s)).collect::<Vec<_>>().join(","));

            // Modify the states depending on the condition
            // If we see a '.' or '#' then it will be added immediately to the states, which may
            // make some switch from "could work" to "invalid".
            // If we see a '?' we fork the state, and keep the the valid parts.
            let next_states: Vec<_> = states.iter().flat_map(|s| {
                // println!(" ... looking at {}{}", Self::format_state(s), *c as u8 as char);
                match c {
                    Condition::Unknown => {
                        let mut tmp = vec![];
                        let mut s1 = s.clone();
                        s1.push(Condition::Damaged);
                        if self.violates_constraints(&s1, true) {
                            // println!(" ... {:?} violates constraints", Self::format_state(&s1));
                        } else {
                            // println!(" ... {:?} is acceptable", Self::format_state(&s1));
                            tmp.push(s1);
                        }

                        let mut s2 = s.clone();
                        s2.push(Condition::Operational);
                        if self.violates_constraints(&s2, true) {
                            // println!(" ... {:?} violates constraints", Self::format_state(&s2));
                        } else {
                            // println!(" ... {:?} is acceptable", Self::format_state(&s2));
                            tmp.push(s2);
                        }
                        tmp
                    },
                    _ => {
                        let mut s1 = s.clone();
                        s1.push(*c);
                        if self.violates_constraints(&s1, true) {
                            // println!(" ... {:?} violates constraints", Self::format_state(&s1));
                            vec![]
                        } else {
                            vec![s1]
                        }
                    }
                }
            }).collect();
            // println!(" ... next_states = {}", next_states.iter().map(|s| self.format_state(s)).collect::<Vec<_>>().join(","));
            if next_states.is_empty() {
                println!("no more states, can return 0 early");
                return 0;
            }
            *states = next_states;
        }

        states.iter().filter(|s| {
            let ok = !self.violates_constraints(s, false);
            // if ok {
            //     println!("{}", Self::format_state(s));
            // }
            ok
        }).count()
    }

    // Check whether the given conditions violates the constraints given by the `damaged_spring_groups`
    // configuration.
    // If `partial` is true then the check accepts conditions that are incomplete and could be completed
    // to a full valid configuration, if it is false then the conditions must match perfectly.
    fn violates_constraints(&self, conditions: &[Condition], partial: bool) -> bool {
        let mut next_spring_group = self.damaged_spring_groups.iter();
        let mut damaged_count = 0;
        for c in conditions {
            if *c == Condition::Operational {
                if damaged_count > 0 {
                    match next_spring_group.next() {
                        Some(expected_damaged_count) => {
                            if *expected_damaged_count != damaged_count {
                                return true;
                            }
                        },
                        None => {
                            return true;
                        },
                    }
                    damaged_count = 0;
                }
            } else if *c == Condition::Damaged {
                damaged_count += 1;
            } else {
                panic!("unexpected condition {:?}", c)
            }
        }

        // We're only checking whether the given conditions could still pass, so at this point
        // we only want "close enough": If we collected some damaged springs, then the next value
        // in the iteration must not be smaller than that.
        // For a "full" check we would also verify that there is nothing left over at the end, and
        // any collected damamged springs would have to match exactly.
        match next_spring_group.next() {
            Some(expected_damaged_count) => {
                if partial {
                    damaged_count > *expected_damaged_count
                } else {
                    damaged_count != *expected_damaged_count || next_spring_group.next().is_some()
                }
            }
            None => {
                damaged_count > 0
            }
        }
    }

    fn format_state(state: &[Condition]) -> String {
        std::str::from_utf8(&state.iter().map(|c| *c as u8).collect::<Vec<u8>>()).unwrap().to_string()
    }

    fn parse_state(s: &str) -> Vec<Condition> {
        s.chars().map(Condition::from).collect()
    }
}

impl std::str::FromStr for ConditionRecord {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, g)) = s.split_once(' ') {
            println!("from_str: p='{}', g='{}'", p, g);
            if let Ok(damaged_spring_groups) = g.split(',').map(usize::from_str).collect::<Result<Vec<usize>, _>>() {
                Ok(ConditionRecord {
                    conditions: ConditionRecord::parse_state(p),
                    damaged_spring_groups,
                })
            } else {
                Err("cannot parse damaged spring groups")
            }
        } else {
            Err("Expected two space-separated parts")
        }
    }
}

fn num_arrangements(input: &str) -> usize {
    input.split('\n').map(ConditionRecord::from_str).map(|cr| cr.unwrap().num_arrangements()).sum()
}

// See https://stackoverflow.com/a/66482767/196315
fn repeat_element<T: Clone>(it: impl Iterator<Item = T>, cnt: usize) -> impl Iterator<Item = T> {
    it.flat_map(move |n| std::iter::repeat(n).take(cnt))
}

pub fn main() {
    match std::fs::read_to_string("day12.input") {
        Ok(input) => {
            println!("num_arrangements = {}", num_arrangements(&input));
            let num_arrangements_part2: usize = input.split('\n').filter_map(|i| {
                if let Some((p, g)) = i.split_once(' ') {
                    format!("{} {}", 
                        repeat_element(std::iter::once(p), 5).collect::<Vec<_>>().join("?"),
                        repeat_element(std::iter::once(g), 5).collect::<Vec<_>>().join(","), 
                    ).parse::<ConditionRecord>().ok()
                } else {
                    None
                }
            }).map(|cr| cr.num_arrangements()).sum();
            println!("num_arrangements (part 2) = {}", num_arrangements_part2);
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static FULLY_KNOWN_CONDITION_RECORDS: &str = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    static DATA: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn condition_record_parse_examples() {
        for line in FULLY_KNOWN_CONDITION_RECORDS.split('\n') {
            if let Err(reason) = line.parse::<ConditionRecord>() {
                panic!("{}: {}", line, reason)
            }
        }
        for line in DATA.split('\n') {
            if let Err(reason) = line.parse::<ConditionRecord>() {
                panic!("{}: {}", line, reason)
            }
        }
    }

    #[test]
    fn condition_record_violates_constraints_partial() {
        assert!(!ConditionRecord::new(&[], &[]).violates_constraints(&[], true), "empty allows empty");
        assert!(!ConditionRecord::new(&[], &[1]).violates_constraints(&[], true), "non-empty allows empty");
        assert!(!ConditionRecord::new(&[], &[1]).violates_constraints(&[Condition::Damaged], true), "must match single");
        assert!(!ConditionRecord::new(&[], &[2,1]).violates_constraints(&[Condition::Damaged, Condition::Damaged, Condition::Operational, Condition::Damaged], true), "must match multiple with operational in the middle");
        assert!(!ConditionRecord::new(&[], &[2,2]).violates_constraints(&[Condition::Damaged, Condition::Damaged, Condition::Operational, Condition::Damaged], true), "last can be incomplete");
    }

    #[test]
    fn condition_record_violates_constraints_partial_small() {
        assert!(ConditionRecord::new(&[], &[3,2,1]).violates_constraints(&[
            Condition::Operational,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Damaged,
            Condition::Operational,
            Condition::Damaged,
            Condition::Operational,
        ], false));
    }

    #[test]
    fn example1_small() {
        assert_eq!("?###???????? 3,2,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 10);
    }
    
    #[test]
    fn example1() {
        assert_eq!("???.### 1,1,3".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!(".??..??...?##. 1,1,3".parse::<ConditionRecord>().unwrap().num_arrangements(), 4);
        assert_eq!("?#?#?#?#?#?#?#? 1,3,1,6".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!("????.#...#... 4,1,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!("????.######..#####. 1,6,5".parse::<ConditionRecord>().unwrap().num_arrangements(), 4);

        assert_eq!(num_arrangements(DATA), 21)
    }

    #[test]
    fn part1_tests() {
        assert_eq!("???.#??#.??? 1,1,2,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 10);
    }
}