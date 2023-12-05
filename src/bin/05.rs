use itertools::Itertools;
use std::ops::Range;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

advent_of_code::solution!(5);

trait MapRange: Sized {
    fn map_range(self, from: Range<Self>, to: Range<Self>) -> Self;
}

impl<T> MapRange for T
where
    T: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>,
{
    fn map_range(self, from: Range<Self>, to: Range<Self>) -> Self {
        let from_range = from.end - from.start;
        let to_range = to.end - to.start;
        let value = self - from.start;
        to.start + (value * to_range) / from_range
    }
}

#[derive(Clone, Debug)]
struct AlmanacMapEntry {
    input_range: Range<u64>,
    output_range: Range<u64>,
}

impl AlmanacMapEntry {
    fn within_range(&self, value: u64) -> bool {
        self.input_range.contains(&value)
    }

    fn map(&self, value: u64) -> u64 {
        value.map_range(self.input_range.clone(), self.output_range.clone())
    }
}

impl FromStr for AlmanacMapEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dest, src, range)) = s.split_whitespace().collect_tuple() {
            let from_start = src.parse::<u64>().map_err(|_| ())?;
            let to_start = dest.parse::<u64>().map_err(|_| ())?;
            let range = range.parse::<u64>().map_err(|_| ())?;

            Ok(AlmanacMapEntry {
                input_range: from_start..from_start + range,
                output_range: to_start..to_start + range,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn new() -> Self {
        AlmanacMap {
            entries: Vec::new(),
        }
    }

    fn push(&mut self, entry: AlmanacMapEntry) {
        self.entries.push(entry);
    }

    fn map(self, value: u64) -> u64 {
        self.entries
            .iter()
            .find(|entry| entry.within_range(value))
            .map(|entry| entry.map(value))
            .unwrap_or(value)
    }
}

#[derive(Clone, Debug)]
enum Seed {
    Spot(u64),
    Range(Range<u64>),
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<Seed>,
    seed_map: AlmanacMap,
    soil_map: AlmanacMap,
    fertilizer_map: AlmanacMap,
    water_map: AlmanacMap,
    sunlight_map: AlmanacMap,
    temperature_map: AlmanacMap,
    humidity_map: AlmanacMap,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_map.clone().map(seed);
        let fertilizer = self.soil_map.clone().map(soil);
        let water = self.fertilizer_map.clone().map(fertilizer);
        let light = self.water_map.clone().map(water);
        let temp = self.sunlight_map.clone().map(light);
        let humidity = self.temperature_map.clone().map(temp);
        let result = self.humidity_map.clone().map(humidity);

        result
    }

    fn min_range_location(&self, range: Range<u64>) -> u64 {
        let mut ranges = vec![range];
        let maps = vec![
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
        ];

        for map_str in maps {
            let mut new_ranges = Vec::new();

            let map = match map_str {
                "seed" => &self.seed_map,
                "soil" => &self.soil_map,
                "fertilizer" => &self.fertilizer_map,
                "water" => &self.water_map,
                "light" => &self.sunlight_map,
                "temperature" => &self.temperature_map,
                "humidity" => &self.humidity_map,
                _ => panic!("Unknown map"),
            };

            while let Some(range) = ranges.pop() {
                let mut broken = false;

                for entry in map.entries.clone() {
                    let overlap_min = u64::max(range.start, entry.input_range.start);
                    let overlap_max = u64::min(range.end, entry.input_range.end);

                    if overlap_min < overlap_max {
                        let overlap = entry.map(overlap_min)..entry.map(overlap_max);
                        new_ranges.push(overlap);

                        if overlap_min > range.start {
                            let new_range = range.start..overlap_min;
                            ranges.push(new_range);
                        }

                        if range.end > overlap_max {
                            let new_range = overlap_max..range.end;
                            ranges.push(new_range);
                        }

                        broken = true;
                        break;
                    }
                }

                if !broken {
                    new_ranges.push(range);
                }
            }

            ranges = new_ranges;
        }

        ranges.iter().map(|r| r.start).min().unwrap()
    }

    fn min_seed_location(&self) -> u64 {
        let mut seed_iter = self.seeds.clone().into_iter();

        let mut min_location = u64::MAX;

        while let Some(seed) = seed_iter.next() {
            match seed {
                Seed::Spot(spot) => {
                    let new_location = self.map_seed_to_location(spot);
                    min_location = u64::min(new_location, min_location);
                }
                Seed::Range(range) => {
                    min_location = u64::min(self.min_range_location(range), min_location);
                }
            }
        }

        min_location
    }

    fn from_str_ranged(s: &str) -> Result<Self, ()> {
        let mut seeds = Vec::new();
        let mut seed_map = AlmanacMap::new();
        let mut soil_map = AlmanacMap::new();
        let mut fertilizer_map = AlmanacMap::new();
        let mut water_map = AlmanacMap::new();
        let mut sunlight_map = AlmanacMap::new();
        let mut temperature_map = AlmanacMap::new();
        let mut humidity_map = AlmanacMap::new();

        let mut iter = s.lines();

        while let Some(line) = iter.next() {
            if line.starts_with("seeds") {
                let (_, seed_str) = line.split(":").collect_tuple().unwrap();

                let parsed_seeds = seed_str
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                seeds = parsed_seeds
                    .iter()
                    .chunks(2)
                    .into_iter()
                    .map(|mut chunk| {
                        let start = *chunk.next().unwrap();
                        let range = *chunk.next().unwrap();
                        Seed::Range(start..(start + range))
                    })
                    .collect();
            } else if line.starts_with("seed-to-soil") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    seed_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("soil") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    soil_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("fertilizer") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    fertilizer_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("water") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    water_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("light") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    sunlight_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("temperature") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    temperature_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("humidity") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    humidity_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            }
        }

        Ok(Almanac {
            seeds,
            seed_map,
            soil_map,
            fertilizer_map,
            water_map,
            sunlight_map,
            temperature_map,
            humidity_map,
        })
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut seed_map = AlmanacMap::new();
        let mut soil_map = AlmanacMap::new();
        let mut fertilizer_map = AlmanacMap::new();
        let mut water_map = AlmanacMap::new();
        let mut sunlight_map = AlmanacMap::new();
        let mut temperature_map = AlmanacMap::new();
        let mut humidity_map = AlmanacMap::new();

        let mut iter = s.lines();

        while let Some(line) = iter.next() {
            if line.starts_with("seeds") {
                let (_, seed_str) = line.split(":").collect_tuple().unwrap();

                seeds = seed_str
                    .split_whitespace()
                    .map(|s| Seed::Spot(s.parse::<u64>().unwrap()))
                    .collect();
            } else if line.starts_with("seed-to-soil") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    seed_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("soil") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    soil_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("fertilizer") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    fertilizer_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("water") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    water_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("light") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    sunlight_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("temperature") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    temperature_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            } else if line.starts_with("humidity") {
                while let Some(line) = iter.next() {
                    if line.is_empty() {
                        break;
                    }

                    humidity_map.push(line.parse::<AlmanacMapEntry>().unwrap());
                }
            }
        }

        Ok(Almanac {
            seeds,
            seed_map,
            soil_map,
            fertilizer_map,
            water_map,
            sunlight_map,
            temperature_map,
            humidity_map,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = input.parse::<Almanac>().unwrap();
    Some(almanac.min_seed_location())
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac::from_str_ranged(input).unwrap();
    Some(almanac.min_seed_location())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
