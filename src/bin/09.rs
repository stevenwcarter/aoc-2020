advent_of_code::solution!(9);

#[cfg(not(test))]
const PREAMBLE_SIZE: usize = 25;
#[cfg(test)]
const PREAMBLE_SIZE: usize = 5;

pub fn find_bad_value(sequence: &[u64]) -> Option<u64> {
    sequence.windows(PREAMBLE_SIZE + 1).find_map(|window| {
        let (preamble, target) = window.split_at(PREAMBLE_SIZE);
        let target = target.first().unwrap();
        let mut found = false;
        for (i, &num1) in preamble.iter().enumerate() {
            for &num2 in &preamble[i + 1..] {
                if num1 + num2 == *target {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            Some(*target)
        } else {
            None
        }
    })
}

pub fn find_bounded_values(sequence: &[u64], bad_value: u64) -> Option<(u64, u64, usize)> {
    let first = sequence.first().unwrap();
    let mut total = *first;

    for (i, val) in sequence[1..].iter().enumerate() {
        total += *val;

        if total == bad_value {
            let min = sequence[0..=i].iter().min().unwrap();
            let max = sequence[0..=i].iter().max().unwrap();
            return Some((*min, *max, i));
        } else if total > bad_value {
            return None;
        }
    }

    None
}

pub fn find_largest_bounded_values(sequence: &[u64], bad_value: u64) -> (u64, u64) {
    let mut first = 0;
    let mut last = 0;
    let mut longest = 0;

    for index in 0..sequence.len() {
        if let Some((start, end, length)) = find_bounded_values(&sequence[index..], bad_value) {
            println!("Found {start} {end} {length}");
            if length > longest {
                first = start;
                last = end;
                longest = length;
            }
        }
    }
    (first, last)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sequence: Vec<u64> = input.lines().filter_map(|line| line.parse().ok()).collect();
    find_bad_value(&sequence)
}

// find sum of smallest and largest number in contiguous set that sums to the invalid number from
// part one
pub fn part_two(input: &str) -> Option<u64> {
    let sequence: Vec<u64> = input.lines().filter_map(|line| line.parse().ok()).collect();
    let bad_value = find_bad_value(&sequence)?;

    let (first, last) = find_largest_bounded_values(&sequence, bad_value);

    Some(first + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }
}
