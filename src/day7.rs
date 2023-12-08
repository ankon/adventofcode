
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
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

trait GameRules {
    fn card_order(&self) -> &str;
    fn classify(&self, hand: &[char]) -> Type;

    fn cmp_game(&self, a: &Game, b: &Game) -> Option<std::cmp::Ordering> {
        print!("comparing {:?} and {:?}: ", a, b);
        // Compare the hand first by their classification
        let a_hand = self.classify(&a.hand);
        let b_hand = self.classify(&b.hand);
        if a_hand != b_hand {
            let result = a_hand.partial_cmp(&b_hand);
            println!("different hand types ({:?} vs {:?}: {:?})", a_hand, b_hand, result);
            return result;
        }
        // If that's still equal, compare the cards, in order.
        for (a_card, b_card) in a.hand.iter().zip(b.hand.iter()) {
            if a_card != b_card {
                let result = self.card_order().find(*a_card).partial_cmp(&self.card_order().find(*b_card));
                println!("different cards ({} vs {}: {:?})", a_card, b_card, result);
                return result;
            }
        }

        // If we get here, the hands are equal.
        println!("equal");
        Some(std::cmp::Ordering::Equal)
    }
}

struct NoJokerRules {}

impl GameRules for NoJokerRules {
    fn card_order(&self) -> &str {
        "23456789TJQKA"
    }

    fn classify(&self, hand: &[char]) -> Type {
        // Count the number of occurrences of each card, and then check
        // the number of occurrences of each number of occurrences.
        let mut counts = [0; 13];
        for card in hand {
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

struct JokerRules {}

impl GameRules for JokerRules {
    fn card_order(&self) -> &str {
        "J23456789TQKA"
    }

    fn classify(&self, hand: &[char]) -> Type {
        // Count the number of occurrences of each card, and then check
        // the number of occurrences of each number of occurrences.
        let mut counts = [0; 13];
        for card in hand {
            match card {
                'A' => counts[12] += 1,
                'K' => counts[11] += 1,
                'Q' => counts[10] += 1,
                'J' => counts[9] += 1,
                'T' => counts[8] += 1,
                _ => counts[card.to_digit(10).unwrap() as usize - 2] += 1,
            }
        }

        // The jokers can be used to replace any card, so we can just add them
        // to the counts as long as we don't use too many of them.
        let joker_count = counts[9];
        print!("joker_count = {}, counts = {:?}", joker_count, counts);
        for (i, count) in counts.iter_mut().enumerate() {
            if i != 9 {
                *count += joker_count;
            }
        }
        println!(" -> {:?}", counts);

        let mut number_of_occurrences = [0; 6];
        for count in &counts {
            number_of_occurrences[*count] += 1;
        }
        println!("number_of_occurrences = {:?}", number_of_occurrences);

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
    fn winnings(&mut self, rules: &dyn GameRules) -> usize {
        // Sort the games by their rank, and then calculate the winnings.
        self.games.sort_by(|a, b| rules.cmp_game(a, b).unwrap());
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
                println!("total winnings (no jokers) = {}", game_list.winnings(&NoJokerRules {}));
                println!("total winnings (jokers) = {}", game_list.winnings(&JokerRules {}));
            }
        },
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn no_joker_rules_classify() {
        let rules = NoJokerRules {};
        assert_eq!(rules.classify(&"AAAAA".chars().collect::<Vec<char>>()), Type::FiveOfAKind);
        assert_eq!(rules.classify(&"AA8AA".chars().collect::<Vec<char>>()), Type::FourOfAKind);
        assert_eq!(rules.classify(&"23332".chars().collect::<Vec<char>>()), Type::FullHouse);
        assert_eq!(rules.classify(&"TTT98".chars().collect::<Vec<char>>()), Type::ThreeOfAKind);
        assert_eq!(rules.classify(&"23432".chars().collect::<Vec<char>>()), Type::TwoPairs);
        assert_eq!(rules.classify(&"A23A4".chars().collect::<Vec<char>>()), Type::OnePair);
        assert_eq!(rules.classify(&"23456".chars().collect::<Vec<char>>()), Type::HighCard);
    }

    #[test]
    fn no_joker_rules_cmp_game() {
        let rules = NoJokerRules {};

        // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger.
        assert!(rules.cmp_game(&Game::new("33332", 0), &Game::new("2AAAA", 0)).unwrap() == std::cmp::Ordering::Greater);
        // Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
        assert!(rules.cmp_game(&Game::new("77888", 0), &Game::new("77788", 0)).unwrap() == std::cmp::Ordering::Greater);

        // From failed attempts:
        assert!(rules.cmp_game(&Game::new("22272", 0), &Game::new("22262", 0)).unwrap() == std::cmp::Ordering::Greater);
        assert!(rules.cmp_game(&Game::new("22262", 0), &Game::new("22272", 0)).unwrap() == std::cmp::Ordering::Less);
    }

    #[test]
    fn no_joker_rules_game_ordering() {
        let mut game_list = GameList { games: vec![
            Game::new("JJJJJ", 0),
            Game::new("2222T", 0),
            Game::new("22272", 0),
            Game::new("22262", 0),
        ]};
        game_list.winnings(&NoJokerRules {});
        assert_eq!(game_list.games, vec![
            Game::new("2222T", 0),
            Game::new("22262", 0),
            Game::new("22272", 0),
            Game::new("JJJJJ", 0),
        ]);
    }

    #[test]
    fn part1_example() {
        assert_eq!(DATA.parse::<GameList>().ok().unwrap().winnings(&NoJokerRules {}), 6440);
    }

    #[test]
    fn joker_rules_classify() {
        let rules = JokerRules {};
        assert_eq!(rules.classify(&"QJJQ2".chars().collect::<Vec<char>>()), Type::FourOfAKind);
        assert_eq!(rules.classify(&"KTJJT".chars().collect::<Vec<char>>()), Type::FourOfAKind);
        assert_eq!(rules.classify(&"T55J5".chars().collect::<Vec<char>>()), Type::FourOfAKind);
    }

    #[test]
    fn joker_rules_cmp_game() {
        let rules = JokerRules {};

        // JKKK2 is weaker than QQQQ2 because J is weaker than Q.
        assert!(rules.cmp_game(&Game::new("JKKK2", 0), &Game::new("QQQQ2", 0)).unwrap() == std::cmp::Ordering::Less);
    }

    #[test]
    fn part2_example() {
        assert_eq!(DATA.parse::<GameList>().ok().unwrap().winnings(&JokerRules {}), 5905);
    }
}
