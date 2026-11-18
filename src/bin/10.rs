use anyhow::{bail, Result};
use std::{cmp::max, fmt::Write};
advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<u32> {
    let mut data = input
        .trim()
        .lines()
        .filter_map(|l| str::parse::<u32>(l.trim()).ok())
        .collect::<Vec<u32>>();

    data.sort_unstable();

    data
}

// Returns (jolts_1, jolts_3)
fn find_jolt_differences(jolts: &[u32]) -> Result<(u32, u32)> {
    if !jolts.is_sorted() {
        bail!("Input must be sorted")
    }
    let mut jolts_1 = 0;
    // rating is always 3 higher at the end
    let mut jolts_3 = 1;

    let mut last_jolt = 0;
    jolts.iter().for_each(|j| {
        let diff = j - last_jolt;
        match diff {
            1 => jolts_1 += 1,
            3 => jolts_3 += 1,
            _ => {}
        };
        last_jolt = *j;
    });

    Ok((jolts_1, jolts_3))
}
fn find_jolt_differences_as_vec(jolts: &[u32]) -> Result<Vec<u32>> {
    if !jolts.is_sorted() {
        bail!("Input must be sorted")
    }

    let mut last_jolt = 0;

    Ok(jolts
        .iter()
        .map(|j| {
            let diff = j - last_jolt;
            last_jolt = *j;
            diff
        })
        .collect())
}

fn sum_run_lengths(jolts: &[u32]) -> Result<u64> {
    if !jolts.is_sorted() {
        bail!("Input must be sorted")
    }
    let jolts = find_jolt_differences_as_vec(&jolts).unwrap();
    let mut diffs = String::new();
    for n in jolts {
        write!(&mut diffs, "{}", n).unwrap();
    }
    write!(&mut diffs, "3").unwrap();

    let mut result: u64 = 1;
    result *= 13u64.pow(diffs.matches("11111").count() as u32);
    let diffs = diffs.replace("11111", "");
    result *= 7u64.pow(diffs.matches("1111").count() as u32);
    let diffs = diffs.replace("1111", "");
    result *= 4u64.pow(diffs.matches("111").count() as u32);
    let diffs = diffs.replace("111", "");
    result *= 2u64.pow(diffs.matches("11").count() as u32);

    Ok(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    let result = find_jolt_differences(&data).unwrap();

    Some(result.0 * result.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);
    //
    // 61681958912 too low

    let result = sum_run_lengths(&data).unwrap();

    assert!(result > 61681958912, "Answer too low");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jolt_differences() {
        let data = parse(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        let result = find_jolt_differences(&data).unwrap();
        assert_eq!(result.0, 7);
        assert_eq!(result.1, 5);
    }
    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(220));
    }
    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(19208));
    }
}
