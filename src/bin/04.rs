use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    scratch_numbers: Vec<u32>,
}

impl Card {
    fn matching_numbers(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|n| self.scratch_numbers.contains(n))
            .count() as u32
    }

    fn points(&self) -> u32 {
        let matches = self.matching_numbers();

        match matches {
            0 => 0,
            _ => 2u32.pow(matches - 1),
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, card_str) = s.split_once(':').ok_or("missing card")?;

        let (winning_numbers, scratch_numbers) =
            card_str.split_once('|').ok_or("missing numbers")?;

        let winning = winning_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| format!("invalid winning numbers: {}", e))?;
        let scratch = scratch_numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| format!("invalid scratch numbers: {}", e))?;
        Ok(Self {
            winning_numbers: winning,
            scratch_numbers: scratch,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.parse::<Card>().unwrap().points())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|l| (l.parse::<Card>().unwrap()))
        .collect::<Vec<Card>>();

    let mut card_nums = BTreeMap::<u32, u32>::new();

    cards.iter().enumerate().for_each(|(idx, card)| {
        let idx = idx as u32;
        let num = *card_nums.get(&idx).unwrap_or(&1u32);
        let card_matches = card.matching_numbers();

        card_nums.entry(idx).or_insert(1);

        for i in 1..=card_matches {
            let new_idx = idx + i;

            match card_nums.entry(new_idx) {
                Entry::Vacant(e) => {
                    e.insert(1u32 + num);
                }
                Entry::Occupied(mut e) => {
                    *e.get_mut() += num;
                }
            }
        }
    });

    card_nums.values().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
