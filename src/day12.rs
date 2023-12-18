use std::fmt::Display;

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

// State when searching for arrangements
struct State {
    conditions: Vec<Condition>,
    unchecked_conditions: Vec<Condition>,
    last_damaged_count: usize,
    last_damaged_spring_groups_index: usize,
    invalid: bool,
}

impl State {
    fn empty() -> State {
        State {
            conditions: vec![],
            unchecked_conditions: vec![],
            last_damaged_count: 0,
            last_damaged_spring_groups_index: 0,
            invalid: false,
        }
    }
    fn and(&self, c: Condition) -> State {
        if self.invalid {
            panic!("cannot 'and' to an invalid state");
        }
        State {
            conditions: self.conditions.clone(),
            unchecked_conditions: vec![c],
            last_damaged_count: self.last_damaged_count,
            last_damaged_spring_groups_index: self.last_damaged_spring_groups_index,
            invalid: false,
        }
    }


    // Check whether the given conditions violates the constraints given by the `damaged_spring_groups`
    // configuration.
    // If `partial` is true then the check accepts conditions that are incomplete and could be completed
    // to a full valid configuration, if it is false then the conditions must match perfectly.
    fn check_constraints(&mut self, damaged_spring_groups: &[usize], partial: bool, print: bool) -> bool {
        // Invalid states stay invalid, and we can skip partial checks when nothing has changed
        // since the last check.
        if self.invalid || (partial && self.unchecked_conditions.is_empty()) {
            if print {
                println!("check_constraints({}): early exit", self);
            }
            return !self.invalid;
        }

        // Process all unchecked conditions
        for c in self.unchecked_conditions.iter() {
            if print {
                print!("check_constraints(): applying {}", c);
            }
            match c {
                Condition::Operational => {
                    // An operational condition was added, so any incomplete damaged group would now be complete and must match exactly
                    // the indicated group.
                    if self.last_damaged_count > 0 {
                        // A group was open, close it and match it.
                        if let Some(expected_damaged_count) = damaged_spring_groups.get(self.last_damaged_spring_groups_index) {
                            let result = *expected_damaged_count == self.last_damaged_count;
                            self.invalid = !result;
                        } else {
                            // State is invalid: keep the constraint check flag on for now, so that the next one
                            // will also flag it.
                            self.invalid = true;
                        }
                        self.last_damaged_count = 0;
                        self.last_damaged_spring_groups_index += 1;
                    }
                },
                Condition::Damaged => {
                    // A damaged condition was added, so we need to check whether that is still fitting the current index.
                    self.last_damaged_count += 1;
                    if let Some(expected_damaged_count) = damaged_spring_groups.get(self.last_damaged_spring_groups_index) {
                        self.invalid = *expected_damaged_count < self.last_damaged_count;
                    } else {
                        // State is invalid: keep the constraint check flag on for now, so that the next one
                        // will also flag it.
                        self.invalid = true;
                    }
                },
                condition => {
                    panic!("unexpected condition {}", condition);
                },
            };
            self.conditions.push(*c);
            if self.invalid {
                if print {
                    println!(": invalid, breaking");
                }
                break;
            } else {
                if print {
                    println!()
                }
            }
        }
        // Mark check as done, and return whether we're now invalid or not.
        self.unchecked_conditions.clear();

        // If we're not doing a partial check, then the group index should point to the last group
        // if the last condition was damaged, or to after the end so that there are no missing groups.
        // println!("... invalid after applying unchecked = {}", self.invalid);
        if !self.invalid && !partial {
            if print {
                print!("... not partial, checking group index {}", self.last_damaged_spring_groups_index);
            }
            match self.conditions.last() {
                Some(Condition::Damaged) => {
                    // Invalid
                    // - if last damage count was not exactly the last group, or
                    // - if not exactly pointing to the last.
                    if let Some(expected_damaged_count) = damaged_spring_groups.get(self.last_damaged_spring_groups_index) {
                        self.invalid = *expected_damaged_count != self.last_damaged_count;
                    }
                    self.invalid |= self.last_damaged_spring_groups_index != damaged_spring_groups.len() - 1;
                },
                _ => {
                    // Empty or last one was operational, invalid if not after the end
                    self.invalid = self.last_damaged_spring_groups_index != damaged_spring_groups.len();
                },
            };
        }

        !self.invalid
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cond_str = std::str::from_utf8(&self.conditions.iter().map(|c| *c as u8).collect::<Vec<u8>>()).unwrap().to_string();
        let unchecked_cond_str = std::str::from_utf8(&self.unchecked_conditions.iter().map(|c| *c as u8).collect::<Vec<u8>>()).unwrap().to_string();
        write!(f, "\"{}\" (ldc = {}, i = {}, invalid = {}, unchecked = {:?})", cond_str, self.last_damaged_count, self.last_damaged_spring_groups_index, self.invalid, unchecked_cond_str)
    }
}

struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_spring_groups: Vec<usize>,
}

impl ConditionRecord {
    pub fn repeat(&self, n: usize) -> ConditionRecord {
        let mut new_conditions = vec![];
        for i in 0..n {
            for c in &self.conditions {
                new_conditions.push(*c);
            }
            if i != n-1 {
                new_conditions.push(Condition::Unknown);
            }
        }
        ConditionRecord {
            conditions: new_conditions,
            damaged_spring_groups: self.damaged_spring_groups.repeat(n),
        }
    }

    pub fn num_arrangements(&self) -> usize {
        self.arrangements().len()
    }

    fn arrangements(&self) -> Vec<String> {
        // Stupid logic: Each '?' can be either '.' or '#'. So, try both, and then proceed
        // and see if _at the end_. Essentially we're building a matching automaton here?????!?!?!!
        // Each state is the conditions we found.
        let states: &mut Vec<State> = &mut vec![];

        // Start out with one empty state:
        states.push(State::empty());
        for c in &self.conditions {
            let next_states: Vec<_> = states.iter().flat_map(|s| {
                // if cfg!(test) {
                //     print!("state = {}, c = {}", s, c);
                // }
                let new_states = match c {
                    Condition::Unknown => {
                        // if cfg!(test) {
                        //     print!(": unknown");
                        // }
                        let mut tmp = vec![];
                        let mut s1 = s.and(Condition::Damaged);
                        if s1.check_constraints(&self.damaged_spring_groups, true, false) {
                            // if cfg!(test) {
                            //     print!(", s1 = {} is valid", s1);
                            // }
                            tmp.push(s1);
                        }
                        let mut s2 = s.and(Condition::Operational);
                        if s2.check_constraints(&self.damaged_spring_groups, true, false) {
                            // if cfg!(test) {
                            //     print!(", s2 = {} is valid", s2);
                            // }
                            tmp.push(s2);
                        }
                        tmp
                    },
                    c => {
                        // if cfg!(test) {
                        //     print!(": definite");
                        // }
                        let mut s1 = s.and(*c);
                        if s1.check_constraints(&self.damaged_spring_groups, true, false) {
                            // if cfg!(test) {
                            //     print!(", s1 = {} is valid", s1);
                            // }
                            vec![s1]
                        } else {
                            vec![]
                        }
                    }
                };
                // if cfg!(test) {
                //     println!();
                // }
                new_states
            }).collect();
            // println!(" ... next_states = {}", next_states.iter().map(|s| self.format_state(s)).collect::<Vec<_>>().join(","));
            if next_states.is_empty() {
                println!("no more states, can return empty early");
                return vec![];
            }
            *states = next_states;
        }

        // Filter out the invalid states
        let mut result = vec![];
        for s in states.iter_mut() {
            let valid = s.check_constraints(&self.damaged_spring_groups, false, false);
            if valid {
                // println!("state = {}: counted, valid at full", s);
                let cond_str = std::str::from_utf8(&s.conditions.iter().map(|c| *c as u8).collect::<Vec<u8>>()).unwrap().to_string();
                result.push(cond_str);
            } else {
                // println!("state = {}: pruned, invalid at full", s);
            }
        }
        result
    }

    fn parse_state(s: &str) -> Vec<Condition> {
        s.chars().map(Condition::from).collect()
    }

    // OLD
    #[cfg(test)]
    pub fn num_arrangements_old(&self) -> usize {
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

        states.iter().filter(|s| !self.violates_constraints(s, false)).count()
    }

    // Check whether the given conditions violates the constraints given by the `damaged_spring_groups`
    // configuration.
    // If `partial` is true then the check accepts conditions that are incomplete and could be completed
    // to a full valid configuration, if it is false then the conditions must match perfectly.
    #[cfg(test)]
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
    // END: OLD
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

fn num_arrangements(input: &str, repeat: usize) -> usize {
    input.split('\n').map(str::parse::<ConditionRecord>).map(|cr| cr.unwrap().repeat(repeat).num_arrangements()).sum()
}

pub fn main() {
    match std::fs::read_to_string("day12.input") {
        Ok(input) => {
            println!("num_arrangements = {}", num_arrangements(&input, 1));
            println!("num_arrangements (part 2) = {}", num_arrangements(&input, 5));
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

    fn build_state(s: &str, damaged_spring_groups: &[usize]) -> Result<State, &'static str> {
        let mut result = State::empty();
        for c in s.chars() {
            let cond = Condition::from(c);
            result = result.and(cond);
            if !result.check_constraints(damaged_spring_groups, true, false) {
                return Err("cannot build state");
            }
        }
        Ok(result)
    }

    #[test]
    fn state_check_constraints_partial() {
        assert!(State::empty().check_constraints(&[], true, false), "empty allows empty");
        let dsg1 = &[1];
        assert!(State::empty().check_constraints(dsg1, true, false), "non-empty allows empty");
        assert!(build_state("#", dsg1).unwrap().check_constraints(dsg1, true, false), "must match single");
        let dsg21 = &[2, 1];
        assert!(build_state("##.#", dsg21).unwrap().check_constraints(dsg21, true, false), "must match multiple with operational in the middle");
        let dsg22 = &[2, 2];
        assert!(build_state("##.#", dsg22).unwrap().check_constraints(dsg22, true, false), "last can be incomplete");
    }

    #[test]
    fn state_check_constraints_small() {
        let dsg321 = &[3, 2, 1];
        assert!(build_state(".###.#.", dsg321).is_err());
    }

    #[test]
    fn example1_small_1() {
        let dsg = &[3, 2, 1];
        let mut s = build_state(".###.##....#", dsg).unwrap();
        assert_eq!(s.last_damaged_count, 1);
        assert_eq!(s.last_damaged_spring_groups_index, 2);
        let valid_at_full = s.check_constraints(dsg, false, false);
        assert!(valid_at_full);
    }

    #[test]
    fn example1_small_2() {
        let dsg = &[1, 1, 3];
        let mut s = build_state(".#...#....###.", dsg).unwrap();
        assert_eq!(s.last_damaged_count, 0);
        assert_eq!(s.last_damaged_spring_groups_index, 3);
        let valid_at_full = s.check_constraints(dsg, false, false);
        assert!(valid_at_full);
    }

    #[test]
    fn example1_part1() {
        assert_eq!("???.### 1,1,3".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!(".??..??...?##. 1,1,3".parse::<ConditionRecord>().unwrap().num_arrangements(), 4);
        assert_eq!("?#?#?#?#?#?#?#? 1,3,1,6".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!("????.#...#... 4,1,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 1);
        assert_eq!("????.######..#####. 1,6,5".parse::<ConditionRecord>().unwrap().num_arrangements(), 4);
        assert_eq!("?###???????? 3,2,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 10);

        assert_eq!(num_arrangements(DATA, 1), 21)
    }

    #[test]
    fn part1_tests() {
        //assert_eq!("???.#??#.??? 1,1,2,1".parse::<ConditionRecord>().unwrap().num_arrangements(), 10);
        let cr = "##???#??#?????????#? 11,6".parse::<ConditionRecord>().unwrap();
        assert_eq!(cr.arrangements(), &[
            "###########..######.",
            "###########...######",
        ]);
    }

    #[test]
    fn condition_record_repeat() {
        let cr = ".# 1".parse::<ConditionRecord>().unwrap().repeat(2);
        assert_eq!(cr.conditions, vec![
            Condition::Operational,
            Condition::Damaged,
            Condition::Unknown,
            Condition::Operational,
            Condition::Damaged,
        ]);
        assert_eq!(cr.damaged_spring_groups, vec![1,1]);
    }

    #[test]
    fn example1_part2() {
        assert_eq!("???.### 1,1,3".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 1);
        assert_eq!(".??..??...?##. 1,1,3".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 16384);
        assert_eq!("?#?#?#?#?#?#?#? 1,3,1,6".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 1);
        assert_eq!("????.#...#... 4,1,1".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 16);
        assert_eq!("????.######..#####. 1,6,5".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 2500);
        assert_eq!("?###???????? 3,2,1".parse::<ConditionRecord>().unwrap().repeat(5).num_arrangements(), 506250);

        assert_eq!(num_arrangements(DATA, 5), 525152)
    }

    #[test]
    fn old_vs_new() {
        match std::fs::read_to_string("day12.input") {
            Ok(input) => {
                for line in input.split('\n') {
                    let cr = line.parse::<ConditionRecord>().unwrap();
                    let old = cr.num_arrangements_old();
                    let new = cr.num_arrangements();
                    assert_eq!(new, old, "old = {}, new = {}", old, new);
                }
            },
            Err(reason) => println!("error = {}", reason)
        }
    }
}
