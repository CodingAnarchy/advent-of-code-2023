use std::collections::HashMap;

advent_of_code::solution!(3);

fn has_symbol_neighbor(chars: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let ymax = chars.len() as isize;
    let xmax = chars[0].len() as isize;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let i = (y as isize + dy).clamp(0, ymax);
            let j = (x as isize + dx).clamp(0, xmax);

            let other = chars.get(i as usize).and_then(|line| line.get(j as usize));
            if let Some(c) = other {
                if !c.is_ascii_digit() && *c != '.' {
                    return true;
                }
            }
        }
    }

    false
}

fn star_neighbor_pos(chars: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(isize, isize)> {
    let ymax = chars.len() as isize;
    let xmax = chars[0].len() as isize;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let i = (y as isize + dy).clamp(0, ymax);
            let j = (x as isize + dx).clamp(0, xmax);

            let other = chars.get(i as usize).and_then(|line| line.get(j as usize));
            if let Some('*') = other {
                return Some((i, j));
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    chars
        .iter()
        .enumerate()
        .fold(0, |sum, (y, line)| {
            let mut line_total = 0;
            let mut current = 0;
            let mut symbol = false;

            for (x, c) in line.iter().enumerate() {
                if c.is_ascii_digit() {
                    if !symbol {
                        symbol = has_symbol_neighbor(&chars, y, x);
                    }
                    current *= 10;
                    current += c.to_digit(10).unwrap();
                }

                if !c.is_ascii_digit() || x == line.len() - 1 {
                    if symbol {
                        line_total += current;
                    }
                    current = 0;
                    symbol = false;
                }
            }

            sum + line_total
        })
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut candidates: HashMap<(isize, isize), Vec<u32>> = HashMap::new();

    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (y, line) in chars.iter().enumerate() {
        let mut current = 0;
        let mut star = None;

        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if star.is_none() {
                    star = star_neighbor_pos(&chars, y, x);
                }
                current *= 10;
                current += c.to_digit(10).unwrap();
            }

            if !c.is_ascii_digit() || x == line.len() - 1 {
                if let Some((y, x)) = star {
                    candidates.entry((y, x)).or_default().push(current);
                }
                current = 0;
                star = None;
            }
        }
    }

    let mut result = 0;

    for cs in candidates.values() {
        if cs.len() == 2 {
            result += cs[0] * cs[1];
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
