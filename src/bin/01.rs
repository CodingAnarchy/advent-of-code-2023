advent_of_code::solution!(1);

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUMS_REVERSED: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn first_digit(input: &str) -> Option<char> {
    input.chars().find(|c| c.is_ascii_digit())
}

fn last_digit(input: &str) -> Option<char> {
    input.chars().rev().find(|c| c.is_ascii_digit())
}

fn first_digit_spelled_out(input: &str) -> Option<u32> {
    let mut input_string = String::from(input);
    let mut tail;

    if let Some(p) = NUMS.iter().position(|n| input_string.starts_with(n)) {
        return Some(p as u32 + 1);
    }

    while let Some((c, tail_bytes)) = input_string.as_bytes().split_first() {
        let c = char::from(*c);
        tail = tail_bytes
            .iter()
            .map(|c| char::from(*c))
            .collect::<String>();

        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap());
        } else if let Some(p) = NUMS.into_iter().position(|n| tail.starts_with(n)) {
            return Some(p as u32 + 1);
        } else {
            input_string = tail.clone();
        }
    }

    None
}

fn last_digit_spelled_out(input: &str) -> Option<u32> {
    let mut input_string = input.chars().rev().collect::<String>();
    let mut tail;

    if let Some(p) = NUMS_REVERSED.iter().position(|n| input_string.starts_with(n)) {
        return Some(p as u32 + 1);
    }

    while let Some((c, tail_bytes)) = input_string.as_bytes().split_first() {
        let c = char::from(*c);
        tail = tail_bytes
            .iter()
            .map(|c| char::from(*c))
            .collect::<String>();

        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap());
        } else if let Some(p) = NUMS_REVERSED.into_iter().position(|n| tail.starts_with(n)) {
            return Some(p as u32 + 1);
        } else {
            input_string = tail.clone();
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let calibration = format!(
                "{}{}",
                first_digit(line).unwrap(),
                last_digit(line).unwrap()
            );
            calibration.parse::<u32>().unwrap_or(0)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let calibration = format!(
                "{}{}",
                first_digit_spelled_out(line).unwrap(),
                last_digit_spelled_out(line).unwrap()
            );
            calibration.parse::<u32>().unwrap_or(0)
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
