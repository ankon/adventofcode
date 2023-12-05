use std::collections::HashMap;

#[derive(Debug)]
struct Scratchcard {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl std::str::FromStr for Scratchcard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("s = {}", s);
        let mut result = Scratchcard { id: 0, numbers: vec![], winning_numbers: vec![] };
        let mut state = 0;
        for tok in s.split(&[' ', ':'][..]) {
            println!("state = {}, tok = {}", state, tok);
            if tok.trim().is_empty() {
                continue;
            }

            if state == 0 {
                if tok == "Card" {
                    continue;
                }
                if let Ok(id) = tok.parse::<u32>() {
                    result.id = id;
                    state = 1;
                } else {
                    return Err("invalid card id")
                }
            } else if state == 1 {
                if let Ok(number) = tok.parse::<u32>() {
                    result.numbers.push(number);
                } else if tok == "|" {
                    state = 2;
                } else {
                    return Err("invalid input")
                }
            } else if state == 2 {
                if let Ok(number) = tok.parse::<u32>() {
                    result.winning_numbers.push(number);
                } else {
                    return Err("invalid input")
                }
            }
        }
        if result.id != 0 && !result.numbers.is_empty() && !result.winning_numbers.is_empty() {
            println!("result = {:?}", result);
            return Ok(result)
        }
        Err("invalid input")
    }
}

impl Scratchcard {
    fn count_winning(&self) -> u32 {
        let mut winning = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                winning += 1;
            }
        }
        winning
    }

    fn count_points(&self) -> u32 {
        let winning = self.count_winning();
        if winning == 0 {
            return 0;
        }
        2_u32.pow(winning - 1)
    }
}

#[derive(Debug)]
struct PileOfScratchcards {
    cards: Vec<Scratchcard>,
}

impl std::str::FromStr for PileOfScratchcards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = vec![];
        for line in s.lines() {
            if let Ok(card) = line.parse::<Scratchcard>() {
                cards.push(card);
            }
        }
        Ok(PileOfScratchcards { cards })
    }
}

impl PileOfScratchcards {
    fn count_points(&self) -> u32 {
        let mut points = 0;
        for card in &self.cards {
            points += card.count_points();
        }
        points
    }

    fn count_cards_part_2(&self) -> usize {
        let mut dups = HashMap::<u32, usize>::new();
        let mut cards = 0;
        for card in &self.cards {
            let mut cards_of_id = 1;
            if let Some(dups_of_id) = dups.get(&card.id) {
                cards_of_id += dups_of_id;
            }
            println!("cards of id {} = {}", card.id, cards_of_id);
            cards += cards_of_id;

            // Distribute the winning cards further
            let winning = card.count_winning();
            println!("winning = {}", winning);
            for i in 1..=winning {
                dups.entry(card.id + i)
                    .and_modify(|e| *e += cards_of_id )
                    .or_insert(cards_of_id);
            }
        }
        cards
    }
}

pub fn main() {
    match std::fs::read_to_string("day4.input") {
        Ok(input) => {
            if let Ok(scratchcards) = input.parse::<PileOfScratchcards>() {
                println!("points = {}", scratchcards.count_points());
                println!("cards = {}", scratchcards.count_cards_part_2());
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_points_example() {
        static DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let scratchcards = DATA.parse::<PileOfScratchcards>().unwrap();
        assert_eq!(scratchcards.count_points(), 13);
    }

    #[test]
    fn count_cards_part_2() {
        static DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let scratchcards = DATA.parse::<PileOfScratchcards>().unwrap();
        assert_eq!(scratchcards.count_cards_part_2(), 30);
    }
}
