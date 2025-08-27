advent_of_code::solution!(12);

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Default)]
pub struct State {
    x: i32,
    y: i32,
    direction: i32,
}

pub struct State2 {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Default for State2 {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }
}

impl State {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::N(value) => self.y += value,
            Instruction::S(value) => self.y -= value,
            Instruction::E(value) => self.x += value,
            Instruction::W(value) => self.x -= value,
            Instruction::L(value) => self.direction = (self.direction - value + 360) % 360,
            Instruction::R(value) => self.direction = (self.direction + value) % 360,
            Instruction::F(value) => match self.direction {
                0 => self.x += value,
                90 => self.y -= value,
                180 => self.x -= value,
                270 => self.y += value,
                _ => unreachable!("Invalid direction {}", self.direction),
            },
        }
    }

    fn manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

impl State2 {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::N(value) => self.waypoint_y += value,
            Instruction::S(value) => self.waypoint_y -= value,
            Instruction::E(value) => self.waypoint_x += value,
            Instruction::W(value) => self.waypoint_x -= value,
            Instruction::L(value) => {
                let times = (value / 90) % 4;
                (0..times).for_each(|_| {
                    let new_x = -self.waypoint_y;
                    let new_y = self.waypoint_x;
                    self.waypoint_x = new_x;
                    self.waypoint_y = new_y;
                });
            }
            Instruction::R(value) => {
                let times = (value / 90) % 4;
                (0..times).for_each(|_| {
                    let new_x = self.waypoint_y;
                    let new_y = -self.waypoint_x;
                    self.waypoint_x = new_x;
                    self.waypoint_y = new_y;
                });
            }
            Instruction::F(value) => {
                self.x += self.waypoint_x * value;
                self.y += self.waypoint_y * value;
            }
        }
    }

    fn manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let value: i32 = value.parse().map_err(|_| ())?;
        match action {
            "N" => Ok(Instruction::N(value)),
            "S" => Ok(Instruction::S(value)),
            "E" => Ok(Instruction::E(value)),
            "W" => Ok(Instruction::W(value)),
            "L" => Ok(Instruction::L(value)),
            "R" => Ok(Instruction::R(value)),
            "F" => Ok(Instruction::F(value)),
            _ => unreachable!("Unknown instruction {action}"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut state = State::default();
    input
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|instruction: Instruction| {
            state.apply_instruction(instruction);
        });

    state.manhattan_distance().into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut state = State2::default();
    input
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|instruction: Instruction| {
            state.apply_instruction(instruction);
        });

    state.manhattan_distance().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
