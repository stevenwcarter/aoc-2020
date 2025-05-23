use std::str::FromStr;

use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(6);

pub struct Input(Vec<Group>);
pub struct Group(Vec<Vec<char>>);

impl FromStr for Input {
    type Err = ();
    fn from_str(input: &str) -> Result<Input, ()> {
        let groups = input
            .split("\n\n")
            .map(|group| Group(group.lines().map(|line| line.chars().collect()).collect()))
            .collect();
        Ok(Input(groups))
    }
}

impl Group {
    pub fn count_unique_answers(&self) -> usize {
        self.0
            .iter()
            .fold(&mut HashSet::new(), |acc, person| {
                person.iter().for_each(|answer| {
                    acc.insert(answer);
                });
                acc
            })
            .len()
    }

    pub fn count_common_answers(&self) -> usize {
        self.0
            .iter()
            .fold(&mut HashMap::new(), |acc, person| {
                person.iter().for_each(|answer| {
                    *acc.entry(answer).or_insert(0) += 1;
                });
                acc
            })
            .values()
            .filter(|&&count| count == self.0.len())
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from_str(input).unwrap();

    Some(
        input
            .0
            .iter()
            .map(|group| group.count_unique_answers())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from_str(input).unwrap();

    Some(
        input
            .0
            .iter()
            .map(|group| group.count_common_answers())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
