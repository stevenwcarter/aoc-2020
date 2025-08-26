advent_of_code::solution!(11);

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone, Debug)]
pub struct Grid {
    positions: Vec<PositionType>,
    next_positions: Vec<PositionType>,
    height: usize,
    width: usize,
    current_position: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            positions: vec![PositionType::Floor; width * height],
            next_positions: vec![PositionType::Floor; width * height],
            width,
            height,
            current_position: 0,
        }
    }
    fn get_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        self.get_index_checked(x as i32, y as i32).unwrap()
    }
    fn get_index_checked(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        Some(y as usize * self.width + x as usize)
    }
    fn get_first_seat_index_in_direction(
        &self,
        dir_x: i32,
        dir_y: i32,
        x: i32,
        y: i32,
    ) -> Option<usize> {
        let (new_x, new_y) = (x + dir_x, y + dir_y);

        let index = self.get_index_checked(new_x, new_y)?;

        match self.get_type_at_index(index) {
            PositionType::Occupied => Some(index),
            PositionType::Empty => Some(index),
            PositionType::Floor => {
                self.get_first_seat_index_in_direction(dir_x, dir_y, new_x, new_y)
            }
        }
    }
    pub fn get_pos(&self, x: usize, y: usize) -> &PositionType {
        let index = self.get_index(x, y);
        self.get_type_at_index(index)
    }
    pub fn get_type_at_index(&self, index: usize) -> &PositionType {
        self.positions.get(index).unwrap()
    }
    pub fn set_pos(&mut self, x: usize, y: usize, position_type: PositionType) {
        let index = self.get_index(x, y);

        *self.positions.get_mut(index).unwrap() = position_type;
    }
    pub fn set_next_pos(&mut self, x: usize, y: usize, position_type: PositionType) {
        let index = self.get_index(x, y);

        *self.next_positions.get_mut(index).unwrap() = position_type;
    }
    pub fn get_adjacent_occupancies_count(&self, x: i32, y: i32) -> usize {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .filter_map(|(x, y)| self.get_index_checked(*x, *y))
        .map(|index| self.get_type_at_index(index))
        .filter(|&&position_type| matches!(position_type, PositionType::Occupied))
        .count()
    }
    pub fn get_adjacent_occupancies_count_2(&self, x: i32, y: i32) -> usize {
        // TODO: get first seat upward, first seat diagonal up right, etc.
        DIRECTIONS
            .iter()
            .filter_map(|(dir_x, dir_y)| {
                self.get_first_seat_index_in_direction(*dir_x, *dir_y, x, y)
            })
            .map(|index| self.get_type_at_index(index))
            .filter(|&&position_type| matches!(position_type, PositionType::Occupied))
            .count()
    }
    pub fn append(&mut self, position_type: PositionType) {
        self.positions[self.current_position] = position_type;
        self.current_position += 1;
    }
    pub fn count_occupied(&self) -> usize {
        self.positions
            .iter()
            .filter(|p| matches!(p, PositionType::Occupied))
            .count()
    }
    // returns changed state count
    pub fn advance_state(&mut self) -> Option<usize> {
        self.next_positions = self.positions.clone();
        let mut change_count = 0;
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let adjacencies = self.get_adjacent_occupancies_count(x as i32, y as i32);
                match (self.get_pos(x, y), adjacencies) {
                    (PositionType::Empty, 0) => {
                        change_count += 1;
                        self.set_next_pos(x, y, PositionType::Occupied);
                        // becomes occupied
                    }
                    (PositionType::Occupied, 4..=8) => {
                        change_count += 1;
                        self.set_next_pos(x, y, PositionType::Empty);
                        // becomes empty
                    }
                    _ => {
                        // no change
                    }
                }
            })
        });
        if change_count > 0 {
            std::mem::swap(&mut self.positions, &mut self.next_positions);

            return Some(change_count);
        }

        None
    }
    pub fn advance_state_2(&mut self) -> Option<usize> {
        let mut change_count = 0;
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let adjacencies = self.get_adjacent_occupancies_count_2(x as i32, y as i32);
                match (self.get_pos(x, y), adjacencies) {
                    (PositionType::Empty, 0) => {
                        change_count += 1;
                        self.set_next_pos(x, y, PositionType::Occupied);
                        // becomes occupied
                    }
                    (PositionType::Occupied, 5..=8) => {
                        change_count += 1;
                        self.set_next_pos(x, y, PositionType::Empty);
                        // becomes empty
                    }
                    (position_type, _) => {
                        self.set_next_pos(x, y, *position_type);
                        // no change
                    }
                }
            })
        });
        if change_count > 0 {
            std::mem::swap(&mut self.positions, &mut self.next_positions);

            return Some(change_count);
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PositionType {
    Occupied,
    Empty,
    Floor,
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();

    let width = lines.first().unwrap().chars().collect::<Vec<_>>().len();
    let height = lines.len();
    let mut grid = Grid::new(width, height);

    lines.iter().for_each(|line| {
        line.chars()
            .map(|c| match c {
                'L' => PositionType::Empty,
                '.' => PositionType::Floor,
                '#' => PositionType::Occupied,
                x => unreachable!("Invalid {x:?} found while parsing"),
            })
            .for_each(|position_type| {
                grid.append(position_type);
            });
    });

    while grid.advance_state().is_some() {
        // advance
    }

    Some(grid.count_occupied())
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();

    let width = lines.first().unwrap().chars().collect::<Vec<_>>().len();
    let height = lines.len();
    let mut grid = Grid::new(width, height);

    lines.iter().for_each(|line| {
        line.chars()
            .map(|c| match c {
                'L' => PositionType::Empty,
                '.' => PositionType::Floor,
                '#' => PositionType::Occupied,
                x => unreachable!("Invalid {x:?} found while parsing"),
            })
            .for_each(|position_type| {
                grid.append(position_type);
            });
    });

    while grid.advance_state_2().is_some() {
        // advance
    }

    Some(grid.count_occupied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
