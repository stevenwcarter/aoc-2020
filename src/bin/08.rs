advent_of_code::solution!(8);

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Debug, Clone)]
pub struct State {
    pub acc: i64,
    pub instructions: Vec<Instruction>,
}

impl State {
    pub fn parse(input: &str) -> Self {
        let instructions = input
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let value = parts[1].parse::<i64>().unwrap();
                match parts[0] {
                    "nop" => Instruction::Noop(value),
                    "acc" => Instruction::Acc(value),
                    "jmp" => Instruction::Jmp(value),
                    _ => panic!("Unknown instruction"),
                }
            })
            .collect();

        Self {
            acc: 0,
            instructions,
        }
    }
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            instructions,
        }
    }

    pub fn run(&mut self, part_2: bool) -> Option<i64> {
        let mut index = 0;
        let mut visited = vec![false; self.instructions.len()];
        while index < self.instructions.len() {
            if visited[index] {
                if part_2 {
                    return None;
                } else {
                    return Some(self.acc);
                }
            }
            visited[index] = true;
            match &self.instructions[index] {
                Instruction::Noop(_) => index += 1,
                Instruction::Acc(value) => {
                    self.acc += value;
                    index += 1;
                }
                Instruction::Jmp(offset) => index = (index as i64 + offset) as usize,
            }
        }
        Some(self.acc)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut state = State::parse(input);

    state.run(false)
}

pub fn part_two(input: &str) -> Option<i64> {
    let state = State::parse(input);
    let mut acc: Option<i64> = None;
    for i in 0..state.instructions.len() {
        if let Instruction::Jmp(value) = state.instructions[i] {
            let mut new_state = state.clone();
            new_state.instructions[i] = Instruction::Noop(value);
            acc = new_state.run(true);
            if acc.is_some() {
                break;
            }
        } else if let Instruction::Noop(value) = state.instructions[i] {
            let mut new_state = state.clone();
            new_state.instructions[i] = Instruction::Jmp(value);
            acc = new_state.run(true);
            if acc.is_some() {
                break;
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
