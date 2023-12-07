use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(7);

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: String,
    card_map: HashMap<char, u64>,
    bid: u64,
}

impl Hand {
    fn has_five_of_a_kind(&self) -> bool {
        self.card_map.values().any(|&x| x == 5)
    }

    fn has_four_of_a_kind(&self) -> bool {
        self.card_map.values().any(|&x| x == 4)
    }

    fn has_full_house(&self) -> bool {
        let mut values = self.card_map.values().collect::<Vec<&u64>>();
        values.sort();
        values == vec![&2, &3]
    }

    fn has_three_of_a_kind(&self) -> bool {
        self.card_map.values().any(|&x| x == 3)
    }

    fn has_two_pairs(&self) -> bool {
        self.card_map.values().filter(|&x| *x == 2).count() == 2
    }

    fn has_one_pair(&self) -> bool {
        self.card_map.values().any(|&x| x == 2)
    }

    fn cmp_cards(&self, other: &Self) -> Ordering {
        let self_cards = self.cards.chars().collect::<Vec<char>>();
        let other_cards = other.cards.chars().collect::<Vec<char>>();

        for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
            let self_card_idx = CARDS.iter().position(|x| x == self_card).unwrap();
            let other_card_idx = CARDS.iter().position(|x| x == other_card).unwrap();

            match self_card_idx.cmp(&other_card_idx) {
                Ordering::Greater => return Ordering::Less,
                Ordering::Less => return Ordering::Greater,
                Ordering::Equal => continue,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.has_five_of_a_kind() {
            if other.has_five_of_a_kind() {
                return self.cmp_cards(other);
            }

            return Ordering::Greater;
        } else if other.has_five_of_a_kind() {
            return Ordering::Less;
        }

        if self.has_four_of_a_kind() {
            if other.has_four_of_a_kind() {
                return self.cmp_cards(other);
            }

            return Ordering::Greater;
        } else if other.has_four_of_a_kind() {
            return Ordering::Less;
        }

        if self.has_full_house() {
            if other.has_full_house() {
                return self.cmp_cards(other);
            }
            return Ordering::Greater;
        } else if other.has_full_house() {
            return Ordering::Less;
        }

        if self.has_three_of_a_kind() {
            if other.has_three_of_a_kind() {
                return self.cmp_cards(other);
            }

            return Ordering::Greater;
        } else if other.has_three_of_a_kind() {
            return Ordering::Less;
        }

        if self.has_two_pairs() {
            if other.has_two_pairs() {
                return self.cmp_cards(other);
            }
            return Ordering::Greater;
        } else if other.has_two_pairs() {
            return Ordering::Less;
        }

        if self.has_one_pair() {
            if other.has_one_pair() {
                return self.cmp_cards(other);
            }
            return Ordering::Greater;
        } else if other.has_one_pair() {
            return Ordering::Less;
        }

        self.cmp_cards(other)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<u64>().unwrap();

        let mut card_map = HashMap::new();
        for card in cards.chars() {
            let count = card_map.entry(card).or_insert(0);
            *count += 1;
        }

        Ok(Hand {
            cards: cards.to_string(),
            card_map,
            bid,
        })
    }
}

#[derive(Debug)]
struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn rank_hands(&mut self) {
        self.hands.sort();
    }

    fn total_winnings(&self) -> u64 {
        let mut winnings = 0;
        for (i, hand) in self.hands.iter().enumerate() {
            winnings += hand.bid * (i + 1) as u64;
        }

        winnings
    }
}

impl FromStr for Hands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.lines().map(|l| l.parse::<Hand>().unwrap()).collect();

        Ok(Hands { hands })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands = input.parse::<Hands>().unwrap();
    hands.rank_hands();
    let winnings = hands.total_winnings();

    Some(winnings)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
