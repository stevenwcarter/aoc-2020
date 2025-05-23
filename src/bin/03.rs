advent_of_code::solution!(3);

pub struct Map {
    data: Vec<Vec<bool>>,
    pos: (usize, usize),
}

impl Map {
    pub fn new(data: Vec<Vec<bool>>) -> Self {
        Self { data, pos: (0, 0) }
    }

    pub fn move_right(&mut self, steps: usize) {
        self.pos.0 = (self.pos.0 + steps) % self.data[0].len();
    }

    pub fn move_down(&mut self, steps: usize) {
        self.pos.1 += steps;
    }

    pub fn is_tree(&self) -> Option<bool> {
        if self.pos.1 >= self.data.len() {
            return None;
        }
        Some(self.data[self.pos.1][self.pos.0])
    }

    pub fn reset(&mut self) {
        self.pos = (0, 0);
    }

    pub fn check_slope(&mut self, right: usize, down: usize) -> u64 {
        let mut tree_count = 0;

        while self.is_tree().is_some() {
            if let Some(true) = self.is_tree() {
                tree_count += 1;
            }
            self.move_right(right);
            self.move_down(down);
        }

        self.reset();

        tree_count
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Map {
        data: input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
        pos: (0, 0),
    };

    Some(map.check_slope(3, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map {
        data: input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
        pos: (0, 0),
    };

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    Some(
        slopes
            .iter()
            .fold(1, |acc, &(right, down)| map.check_slope(right, down) * acc),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
