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

// State when searching for arrangements
struct State {
    conditions: Vec<Condition>,
    last_damaged_count: usize,
    last_damaged_spring_groups_index: usize,
    invalid: bool,
    needs_constraint_check: bool,
}

impl State {
    fn empty() -> State {
        State {
            conditions: vec![],
            last_damaged_count: 0,
            last_damaged_spring_groups_index: 0,
            invalid: false,
            needs_constraint_check: false,
        }
    }
    fn and(&self, c: Condition) -> State {
        if self.invalid {
            panic!("cannot 'and' to an invalid state");
        }
        let mut new_conditions = self.conditions.clone();
        new_conditions.push(c);
        State {
            conditions: new_conditions,
            last_damaged_count: self.last_damaged_count,
            last_damaged_spring_groups_index: self.last_damaged_spring_groups_index,
            invalid: false,
            needs_constraint_check: true,
        }
    }


    // Check whether the given conditions violates the constraints given by the `damaged_spring_groups`
    // configuration.
    // If `partial` is true then the check accepts conditions that are incomplete and could be completed
    // to a full valid configuration, if it is false then the conditions must match perfectly.
    fn check_constraints(&mut self, damaged_spring_groups: &[usize], partial: bool) -> bool {
        // Invalid states stay invalid, and we can skip checks when nothing has changed
        // since the last check.
        if self.invalid || !self.needs_constraint_check {
            return !self.invalid;
        }

        // needs_constraint_check is true on a state that recently got a new condition, so we check
        // whether the last condition is not breaking anything.
        match self.conditions.last() {
            Some(Condition::Operational) => {
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
            Some(Condition::Damaged) => {
                // A damaged condition was added, so we need to check whether that is still fitting the current index.
                self.last_damaged_count += 1;
                if let Some(expected_damaged_count) = damaged_spring_groups.get(self.last_damaged_spring_groups_index) {
                    let result = if partial {
                        *expected_damaged_count >= self.last_damaged_count
                    } else {
                        *expected_damaged_count == self.last_damaged_count
                    };
                    self.invalid = !result;
                } else {
                    // State is invalid: keep the constraint check flag on for now, so that the next one
                    // will also flag it.
                    self.invalid = true;
                }
            },
            Some(condition) => {
                panic!("unexpected condition {}", condition);
            },
            None => {
                // Nothing to do really, the thing is empty. But: How did you get here?
                panic!("huh? state is empty");
            },
        }

        // If we're not doing a partial check, then the group index should point after the end
        // of the groups, i.e. there is no missing group.
        if !self.invalid && !partial {
            self.invalid = self.last_damaged_spring_groups_index >= damaged_spring_groups.len();
        }

        // Mark check as done, and return whether we're now invalid or not.
        self.needs_constraint_check = false;
        !self.invalid
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cond_str = std::str::from_utf8(&self.conditions.iter().map(|c| *c as u8).collect::<Vec<u8>>()).unwrap().to_string();
        write!(f, "\"{}\" (ldc = {}, i = {}", cond_str, self.last_damaged_count, self.last_damaged_spring_groups_index)
    }
}

struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_spring_groups: Vec<usize>,
}

impl ConditionRecord {
    pub fn num_arrangements(&self) -> usize {
        // Stupid logic: Each '?' can be either '.' or '#'. So, try both, and then proceed
        // and see if _at the end_. Essentially we're building a matching automaton here?????!?!?!!
        // Each state is the conditions we found.
        let states: &mut Vec<State> = &mut vec![];

        // Start out with one empty state:
        states.push(State::empty());
        for c in &self.conditions {
            let next_states: Vec<_> = states.iter().flat_map(|s| {
                match c {
                    Condition::Unknown => {
                        let mut tmp = vec![];
                        let mut s1 = s.and(Condition::Damaged);
                        if s1.check_constraints(&self.damaged_spring_groups, true) {
                            tmp.push(s1);
                        }
                        let mut s2 = s.and(Condition::Operational);
                        if s2.check_constraints(&self.damaged_spring_groups, true) {
                            tmp.push(s2);
                        }
                        tmp
                    },
                    c => {
                        let mut s1 = s.and(*c);
                        if s1.check_constraints(&self.damaged_spring_groups, true) {
                            vec![s1]
                        } else {
                            vec![]
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

        let mut result = 0;
        for s in states.iter_mut() {
            if s.check_constraints(&self.damaged_spring_groups, false) {
                result += 1;
            }
        }
        result
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

    fn build_state(s: &str, damaged_spring_groups: &[usize]) -> Result<State, &'static str> {
        let mut result = State::empty();
        for c in s.chars() {
            let cond = Condition::from(c);
            result = result.and(cond);
            if !result.check_constraints(damaged_spring_groups, true) {
                return Err("cannot build state");
            }
        }
        Ok(result)
    }

    #[test]
    fn state_check_constraints_partial() {
        assert!(State::empty().check_constraints(&[], true), "empty allows empty");
        let dsg1 = &[1];
        assert!(State::empty().check_constraints(dsg1, true), "non-empty allows empty");
        assert!(build_state("#", dsg1).unwrap().check_constraints(dsg1, true), "must match single");
        let dsg21 = &[2, 1];
        assert!(build_state("##.#", dsg21).unwrap().check_constraints(dsg21, true), "must match multiple with operational in the middle");
        let dsg22 = &[2, 2];
        assert!(build_state("##.#", dsg22).unwrap().check_constraints(dsg22, true), "last can be incomplete");
    }

    #[test]
    fn state_check_constraints_small() {
        let dsg321 = &[3, 2, 1];
        assert!(build_state(".###.#.", dsg321).is_err());
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