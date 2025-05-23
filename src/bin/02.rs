advent_of_code::solution!(2);

pub struct Entry<'a> {
    pub start: u8,
    pub end: u8,
    pub letter: char,
    pub password: &'a str,
}

impl<'a> Entry<'a> {
    pub fn new(line: &'a str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let range: Vec<u8> = parts[0].split('-').map(|s| s.parse().unwrap()).collect();
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2];

        Self {
            start: range[0],
            end: range[1],
            letter,
            password,
        }
    }

    pub fn is_valid_a(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.start as usize && count <= self.end as usize
    }

    pub fn is_valid_b(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth((self.start - 1) as usize)
            .unwrap();
        let second = self.password.chars().nth((self.end - 1) as usize).unwrap();
        (first == self.letter) ^ (second == self.letter)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Entry::new)
            .filter(|entry| entry.is_valid_a())
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Entry::new)
            .filter(|entry| entry.is_valid_b())
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
