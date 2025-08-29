use hashbrown::HashMap;
use nom::Parser;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
