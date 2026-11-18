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

impl Data {
    /// Whether `value` is valid for at least one attribute.
    fn is_valid_value(&self, value: u32) -> bool {
        self.attributes.iter().any(|a| a.contains(value))
    }

    /// Works out which ticket position each attribute occupies: `result[i]` is the position
    /// of `attributes[i]`.
    fn attribute_positions(&self) -> Vec<usize> {
        let valid_tickets: Vec<&Ticket> = self
            .other_tickets
            .iter()
            .filter(|t| t.iter().all(|&v| self.is_valid_value(v)))
            .collect();
        let width = self.your_ticket.len();
        assert!(width <= 64, "tickets should have at most 64 fields");

        // Per attribute, a bitmask of the positions whose values all fit that attribute.
        let mut candidates: Vec<u64> = self
            .attributes
            .iter()
            .map(|a| {
                (0..width)
                    .filter(|&pos| valid_tickets.iter().all(|t| a.contains(t[pos])))
                    .fold(0, |mask, pos| mask | 1 << pos)
            })
            .collect();

        // Repeatedly pin down an attribute with a single remaining candidate and remove that
        // position from every attribute (which also zeroes the pinned attribute's own mask).
        let mut positions = vec![0; candidates.len()];
        for _ in 0..candidates.len() {
            let index = candidates
                .iter()
                .position(|c| c.count_ones() == 1)
                .expect("some attribute should have exactly one candidate position");
            let mask = candidates[index];
            positions[index] = mask.trailing_zeros() as usize;
            for c in &mut candidates {
                *c &= !mask;
            }
        }
        positions
    }

    /// Multiplies the values on your ticket for every attribute whose name starts with `prefix`.
    fn your_ticket_product(&self, prefix: &str) -> u64 {
        self.attributes
            .iter()
            .zip(self.attribute_positions())
            .filter(|(a, _)| a.name.starts_with(prefix))
            .map(|(_, pos)| u64::from(self.your_ticket[pos]))
            .product()
    }
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

    Some(
        data.other_tickets
            .iter()
            .flatten()
            .copied()
            .filter(|&value| !data.is_valid_value(value))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input).your_ticket_product("departure"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71));
    }

    fn part_two_example() -> Data {
        parse_input(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ))
    }

    #[test]
    fn test_attribute_positions() {
        // Attributes are `class`, `row`, `seat`; the ticket order is `row`, `class`, `seat`.
        assert_eq!(part_two_example().attribute_positions(), vec![1, 0, 2]);
    }

    #[test]
    fn test_your_ticket_product() {
        let data = part_two_example();
        assert_eq!(data.your_ticket_product("class"), 12);
        assert_eq!(data.your_ticket_product("row"), 11);
        // An empty prefix matches every attribute.
        assert_eq!(data.your_ticket_product(""), 11 * 12 * 13);
    }
}
