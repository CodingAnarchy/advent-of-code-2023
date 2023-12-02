use std::cmp::max;
use std::str::FromStr;

advent_of_code::solution!(2);

const MAX_PICKS: Pick = Pick {
    blue: 14,
    green: 13,
    red: 12,
};

#[derive(Debug)]
struct Pick {
    blue: u32,
    green: u32,
    red: u32,
}

impl Pick {
    fn possible(&self) -> bool {
        self.blue <= MAX_PICKS.blue && self.green <= MAX_PICKS.green && self.red <= MAX_PICKS.red
    }

    fn min_add(&self, other: &Pick) -> Pick {
        Pick {
            blue: max(self.blue, other.blue),
            green: max(self.green, other.green),
            red: max(self.red, other.red),
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

impl FromStr for Pick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let mut pick = Pick {
            blue: 0,
            green: 0,
            red: 0,
        };

        while let Some(part) = parts.next() {
            let (count, color) = part.trim().split_once(' ').unwrap();
            match color {
                "blue" => pick.blue = count.parse().unwrap(),
                "green" => pick.green = count.parse().unwrap(),
                "red" => pick.red = count.parse().unwrap(),
                _ => panic!("Unknown color: {}", color),
            }
        }

        Ok(pick)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .filter_map(|line| {
            let (game, picks) = line.split_once(':').unwrap();
            let game_num = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
            if picks.split(';').all(|pick| {
                let pick = pick.parse::<Pick>().unwrap();
                pick.possible()
            }) {
                return Some(game_num);
            } else {
                return None;
            }
        })
        .sum();

    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let empty = Pick {
                blue: 0,
                green: 0,
                red: 0,
            };

            let game_picks = line.split_once(':').unwrap().1;
            let picks = game_picks
                .split(';')
                .map(|pick| pick.parse::<Pick>().unwrap());
            let min_pick = picks.fold(empty, |acc, pick| acc.min_add(&pick));
            min_pick.power()
        })
        .sum();

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
