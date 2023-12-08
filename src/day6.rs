struct RaceTable {
    races: Vec<(usize, usize)>,
}

impl std::str::FromStr for RaceTable {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((times_line, distances_line)) = s.split_once('\n') {
            let times = times_line.split_ascii_whitespace().skip(1).map(|s| s.parse::<usize>()).collect::<Result<Vec<usize>, _>>().map_err(|_| "cannot parse times")?;
            let distances = distances_line.split_ascii_whitespace().skip(1).map(|s| s.parse::<usize>()).collect::<Result<Vec<usize>, _>>().map_err(|_| "cannot parse distances")?;
            if times.len() != distances.len() {
                return Err("mismatch between times and distances")
            }
            return Ok(RaceTable { races: times.iter().zip(distances.iter()).map(|(time, distance)| (*time, *distance)).collect() })
        }
        Err("cannot parse")
    }
}

impl RaceTable {
    fn number_of_ways_to_beat_the_record(time: usize, distance: usize) -> usize {
        // The distance is the record to beat, and we do that by by checking for each possible
        // time of holding the button (x) the value of x * (time - x) > distance.
        // Instead of running over the values, we can also find the two zeros of the function -x^2 - x*time - distance = 0: The number of
        // times we beat the record is then floor(x2 - x1). We know that x1 is >= 0; there is a chance
        // that x2 is > time, in which case we have to use time instead of x2.
        //
        // -x^2 + time * x - distance = 0 | * -1
        //  x^2 - time * x + distance = 0
        //
        // The zeros of the function are x{1,2} = time/2 +- sqrt(time^2/4 - distance).
        //
        // The tricky part here is that if the zero is not an integer, we have to round it up (x1) or down (x2),
        // and if it is an integer we need to exclude it from the result and use the next higher/lower value.
        // We can do that by adding 1 and then rounding down (x1) or subtracting 1 and then rounding up (x2).
        let m = ((time * time / 4 - distance) as f32).sqrt();
        let x1 = ((time as f32) / 2.0 - m + 1_f32).floor().clamp(0_f32, time as f32);
        let x2 = ((time as f32) / 2.0 + m - 1_f32).ceil().clamp(0_f32, time as f32);
        let result = (x2 - x1 + 1_f32).floor() as usize;
        println!("time = {}, distance = {}, m = {}, x1 = {}, x2 = {}: result = {}", time, distance, m, x1, x2, result);
        result
    }

    fn product(&self) -> usize {
        let mut result = 1;
        for (time, distance) in &self.races {
            let number_of_ways = Self::number_of_ways_to_beat_the_record(*time, *distance);
            result *= number_of_ways;
        }
        result
    }
}

pub fn main() {
    match std::fs::read_to_string("day6.input") {
        Ok(input) => {
            if let Ok(race_table) = input.parse::<RaceTable>() {
                println!("product of number of ways to win races (part 1) = {}", race_table.product());
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
        static DATA: &str = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(DATA.parse::<RaceTable>().ok().unwrap().product(), 288);
    }
}
