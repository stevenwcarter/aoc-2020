use std::ops::RangeInclusive;

advent_of_code::solution!(16);

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub ranges: [RangeInclusive<u32>; 2],
}

impl Attribute {
    fn contains(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

pub type Ticket = Vec<u32>;

#[derive(Debug)]
pub struct Data {
    pub attributes: Vec<Attribute>,
    pub your_ticket: Ticket,
    pub other_tickets: Vec<Ticket>,
}

/// Parses a range like `1-3`.
fn parse_range(range: &str) -> RangeInclusive<u32> {
    let (min, max) = range
        .trim()
        .split_once('-')
        .expect("range should be `min-max`");
    let min: u32 = min.parse().expect("range min should be a number");
    let max: u32 = max.parse().expect("range max should be a number");
    min..=max
}

/// Parses a rule line like `class: 1-3 or 5-7`.
fn parse_attribute(line: &str) -> Attribute {
    let (name, ranges) = line
        .split_once(':')
        .expect("attribute should be `name: ranges`");
    let (first, second) = ranges
        .split_once(" or ")
        .expect("attribute should have two ranges separated by ` or `");

    Attribute {
        name: name.to_string(),
        ranges: [parse_range(first), parse_range(second)],
    }
}

/// Parses a comma-separated ticket line like `7,1,14`.
fn parse_ticket(line: &str) -> Ticket {
    line.split(',')
        .map(|n| n.parse().expect("ticket value should be a number"))
        .collect()
}

fn parse_input(input: &str) -> Data {
    let mut sections = input.split("\n\n");
    let mut next_section = |name| {
        sections
            .next()
            .unwrap_or_else(|| panic!("input should have a `{name}` section"))
    };

    let attributes = next_section("attributes")
        .lines()
        .map(parse_attribute)
        .collect();

    // The ticket sections each start with a header line (`your ticket:` / `nearby tickets:`).
    let your_ticket = next_section("your ticket")
        .lines()
        .nth(1)
        .map(parse_ticket)
        .expect("`your ticket` section should contain a ticket");
    let other_tickets = next_section("nearby tickets")
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    Data {
        attributes,
        your_ticket,
        other_tickets,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);
    let is_invalid = |value: &u32| !data.attributes.iter().any(|a| a.contains(*value));

    Some(
        data.other_tickets
            .iter()
            .flatten()
            .copied()
            .filter(is_invalid)
            .sum(),
    )
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
        assert_eq!(result, Some(71));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
