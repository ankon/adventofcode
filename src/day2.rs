#[derive(Debug, Clone)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl std::str::FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_once(':');
        if parts.is_none() {
            return Err("Cannot find separator in input")
        }
        let id_part = parts.unwrap().0.split_once(' ');
        if id_part.is_none() {
            return Err("Cannot find separator in id part")
        } else if id_part.unwrap().0 != "Game" {
            return Err("Not a game?")
        }
        let id = id_part.unwrap().1.parse::<u32>().unwrap();

        // Rest is `;` separated instances, and in there `,` separated `NUM COLOR` pairs.
        let mut draws = vec![];
        for s in parts.unwrap().1.split(';') {
            let mut draw = Draw { red: 0, green: 0, blue: 0 };
            for color_draw in s.split(',') {
                match color_draw.trim_start().split_once(' ') {
                    Some((count, "red")) => draw.red = count.parse::<u32>().unwrap(),
                    Some((count, "green")) => draw.green = count.parse::<u32>().unwrap(),
                    Some((count, "blue")) => draw.blue = count.parse::<u32>().unwrap(),
                    _ => {
                        return Err("Not a known color")
                    }
                }
            }

            draws.push(draw)
        }

        Ok(Game { id, draws })
    }
}

impl Game {
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for draw in self.draws.iter() {
            if draw.red > red || draw.green > green || draw.blue > blue {
                return false
            }
        }
        true
    }
}

fn sum_of_possible_games(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        match line.parse::<Game>() {
            Ok(game) => {
                if game.is_possible(12, 13, 14) {
                    result += game.id;
                }
            },
            Err(reason) => println!("error = {}", reason)
        }
    }
    result
}

fn sum_of_minimal_powers(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        match line.parse::<Game>() {
            Ok(game) => {
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                for draw in game.draws {
                    if draw.red > 0 {
                        max_red = std::cmp::max(max_red, draw.red);
                    }
                    if draw.green > 0 {
                        max_green = std::cmp::max(max_green, draw.green);
                    }
                    if draw.blue > 0 {
                        max_blue = std::cmp::max(max_blue, draw.blue)
                    }
                }
                result += max_red * max_green * max_blue;
            },
            Err(reason) => println!("error = {}", reason)
        }
    }
    result
}

pub fn main() {
    match std::fs::read_to_string("day2.input") {
        Ok(input) => println!("part1 = {}, part2 = {}", sum_of_possible_games(&input), sum_of_minimal_powers(&input)),
        Err(reason) => println!("error = {}", reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_possible_games_example1() {
        static DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_of_possible_games(DATA), 8);
    }

    #[test]
    fn sum_of_minimal_powers_example2() {
        static DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_of_minimal_powers(DATA), 2286);
    }
}
