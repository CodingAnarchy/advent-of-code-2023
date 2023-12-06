use std::iter::zip;
use std::str::FromStr;

advent_of_code::solution!(6);

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn min_hold(&self) -> u64 {
        (1..self.time)
            .find(|t| t * (self.time - t) > self.distance)
            .unwrap_or(self.time)
    }

    fn max_hold(&self) -> u64 {
        (1..self.time)
            .rfind(|t| t * (self.time - t) > self.distance)
            .unwrap_or(0)
    }

    fn num_ways_to_win(&self) -> u64 {
        self.max_hold() - self.min_hold() + 1
    }
}

impl FromStr for Race {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = 0;
        let mut distance = 0;
        let iter = s.lines();

        for line in iter {
            if line.is_empty() {
                break;
            }

            if line.starts_with("Time") {
                time = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
            } else if line.starts_with("Distance") {
                distance = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
            }
        }

        Ok(Race { time, distance })
    }
}

struct Races {
    races: Vec<Race>,
}

impl Races {
    fn product(&self) -> u64 {
        self.races.iter().map(|r| r.num_ways_to_win()).product()
    }
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut times = Vec::new();
        let mut distances = Vec::new();

        let iter = s.lines();

        for line in iter {
            if line.is_empty() {
                break;
            }

            if line.starts_with("Time") {
                let times_str = line.split(':').nth(1).unwrap().trim();

                times = times_str
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            } else if line.starts_with("Distance") {
                let distances_str = line.split(':').nth(1).unwrap().trim();
                distances = distances_str
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            }
        }

        let races = zip(times, distances)
            .map(|(time, dist)| Race {
                time,
                distance: dist,
            })
            .collect();

        Ok(Races { races })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = input.parse::<Races>().unwrap();
    Some(races.product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = input.parse::<Race>().unwrap();
    Some(race.num_ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
