
#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Game {
    hand: Vec<char>,
    bid: usize,
}

impl Game {
    #[cfg(test)]
    fn new(hand: &str, bid: usize) -> Self {
        Game { hand: hand.chars().collect(), bid }
    }

    fn classify(&self) -> Type {
        // Count the number of occurrences of each card, and then check
        // the number of occurrences of each number of occurrences.
        let mut counts = [0; 13];
        for card in &self.hand {
            match card {
                'A' => counts[12] += 1,
                'K' => counts[11] += 1,
                'Q' => counts[10] += 1,
                'J' => counts[9] += 1,
                'T' => counts[8] += 1,
                _ => counts[card.to_digit(10).unwrap() as usize - 2] += 1,
            }
        }
        let mut number_of_occurrences = [0; 6];
        for count in &counts {
            number_of_occurrences[*count] += 1;
        }
        if number_of_occurrences[5] == 1 {
            return Type::FiveOfAKind;
        }
        if number_of_occurrences[4] == 1 {
            return Type::FourOfAKind;
        }
        if number_of_occurrences[3] == 1 && number_of_occurrences[2] == 1 {
            return Type::FullHouse;
        }
        if number_of_occurrences[3] == 1 {
            return Type::ThreeOfAKind;
        }
        if number_of_occurrences[2] == 2 {
            return Type::TwoPairs;
        }
        if number_of_occurrences[2] == 1 {
            return Type::OnePair;
        }
        Type::HighCard
    }
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl std::cmp::PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        static CARD_ORDER: &str = "23456789TJQKA";

        print!("comparing {:?} and {:?}: ", self, other);
        // Compare the hand first by their classification
        let self_hand = self.classify();
        let other_hand = other.classify();
        if self_hand != other_hand {
            let result = self_hand.partial_cmp(&other_hand);
            println!("different hand types ({:?} vs {:?}: {:?})", self_hand, other_hand, result);
            return result;
        }
        // If that's still equal, compare the cards, in order.
        for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
            if self_card != other_card {
                let result = CARD_ORDER.find(*self_card).partial_cmp(&CARD_ORDER.find(*other_card));
                println!("different cards ({} vs {}: {:?})", self_card, other_card, result);
                return result;
            }
        }

        // If we get here, the hands are equal.
        println!("equal");
        Some(std::cmp::Ordering::Equal)
    }
}

struct GameList {
    games: Vec<Game>
}

impl std::str::FromStr for GameList {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut games = Vec::new();
        for line in s.split('\n') {
            if let Some((hand, bid)) = line.split_once(' ') {
                let bid = bid.parse::<usize>().map_err(|_| "cannot parse bid")?;
                let hand = hand.chars().collect::<Vec<char>>();
                games.push(Game { hand, bid });
            }
        }
        Ok(GameList { games })
    }
}

impl GameList {
    fn winnings(&mut self) -> usize {
        // Sort the games by their rank, and then calculate the winnings.
        self.games.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("games = {:?}", self.games);

        let mut result = 0;
        for (rank, game) in self.games.iter().enumerate() {
            result += (rank + 1) * game.bid;
        }
        result
    }
}

pub fn main() {
    match std::fs::read_to_string("day7.input") {
        Ok(input) => {
            if let Ok(mut game_list) = input.parse::<GameList>() {
                println!("total winnings (part 1) = {}", game_list.winnings());
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_classify_tests() {
        assert_eq!(Game::new("AAAAA", 0).classify(), Type::FiveOfAKind);
        assert_eq!(Game::new("AA8AA", 0).classify(), Type::FourOfAKind);
        assert_eq!(Game::new("23332", 0).classify(), Type::FullHouse);
        assert_eq!(Game::new("TTT98", 0).classify(), Type::ThreeOfAKind);
        assert_eq!(Game::new("23432", 0).classify(), Type::TwoPairs);
        assert_eq!(Game::new("A23A4", 0).classify(), Type::OnePair);
        assert_eq!(Game::new("23456", 0).classify(), Type::HighCard);
    }

    #[test]
    fn game_partial_ord_tests() {
        // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger.
        assert!(Game::new("33332", 0) > Game::new("2AAAA", 0));
        // Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
        assert!(Game::new("77888", 0) > Game::new("77788", 0));

        // From failed attempts:
        assert!(Game::new("22272", 0) > Game::new("22262", 0));
        assert!(Game::new("22262", 0) < Game::new("22272", 0));
    }

    #[test]
    fn game_list_ordering_tests() {
        let mut game_list = GameList { games: vec![
            Game::new("JJJJJ", 0),
            Game::new("2222T", 0),
            Game::new("22272", 0),
            Game::new("22262", 0),
        ]};
        game_list.winnings();
        assert_eq!(game_list.games, vec![
            Game::new("2222T", 0),
            Game::new("22262", 0),
            Game::new("22272", 0),
            Game::new("JJJJJ", 0),
        ]);
    }

    #[test]
    fn part1_example() {
        static DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(DATA.parse::<GameList>().ok().unwrap().winnings(), 6440);
    }
}
