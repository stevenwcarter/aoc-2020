use hashbrown::HashMap;
use nom::Parser;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};
use std::iter;

advent_of_code::solution!(14);

pub enum Instruction {
    Mask(String),
    Mem(usize, u64),
}

fn apply_mask(value: u64, mask: &str) -> u64 {
    let mut result = value;
    mask.chars().rev().enumerate().for_each(|(i, c)| match c {
        '0' => result &= !(1 << i),
        '1' => result |= 1 << i,
        'X' => {}
        _ => panic!("Invalid mask character: {}", c),
    });

    result
}

fn parse_mem_with_nom(input: &str) -> Instruction {
    fn parse_mem(input: &str) -> IResult<&str, Instruction> {
        let (input, (_, address, _, value)) =
            (tag("mem["), digit1, tag("] = "), digit1).parse(input)?;

        let address = address.parse::<usize>().unwrap();
        let value = value.parse::<u64>().unwrap();

        Ok((input, Instruction::Mem(address, value)))
    }

    let (_, instruction) = parse_mem(input).unwrap();
    instruction
}

fn parse_instruction(input: &str) -> Instruction {
    if let Some(mask) = input.strip_prefix("mask = ") {
        Instruction::Mask(mask.to_owned())
    } else {
        parse_mem_with_nom(input)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
    let mut mask = String::new();

    let mut values: HashMap<usize, u64> = HashMap::new();
    lines
        .iter()
        .map(|l| parse_instruction(l))
        .for_each(|instr| match instr {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(address, value) => {
                values.insert(address, apply_mask(value, &mask));
            }
        });

    values.values().sum::<u64>().into()
}

/// Splits a mask into its `1` bits and its floating (`X`) bits.
fn address_mask_bits(mask: &str) -> (u64, u64) {
    mask.chars().fold((0u64, 0u64), |(ones, floating), c| {
        let (one, float) = match c {
            '0' => (0, 0),
            '1' => (1, 0),
            'X' => (0, 1),
            _ => panic!("Invalid mask character: {}", c),
        };
        ((ones << 1) | one, (floating << 1) | float)
    })
}

/// Yields every address the version 2 decoder writes to for `address`.
///
/// The floating bits are cleared from the base address, then each subset of
/// them is OR-ed back in. `(subset - 1) & floating` steps through the subsets
/// of `floating` in descending order, ending at the empty subset.
fn floating_addresses(address: u64, mask: &str) -> impl Iterator<Item = u64> {
    let (ones, floating) = address_mask_bits(mask);
    let base = (address | ones) & !floating;

    iter::successors(Some(floating), move |&subset| {
        (subset != 0).then(|| (subset - 1) & floating)
    })
    .map(move |subset| base | subset)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mask = String::new();

    let mut values: HashMap<u64, u64> = HashMap::new();
    input
        .lines()
        .map(|l| parse_instruction(l.trim()))
        .for_each(|instr| match instr {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(address, value) => {
                values.extend(floating_addresses(address as u64, &mask).map(|a| (a, value)));
            }
        });

    values.values().sum::<u64>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_two() {
        let sample = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(sample);
        assert_eq!(result, Some(208));
    }
}
