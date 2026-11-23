advent_of_code::solution!(23);

const PART_ONE_MOVES: usize = 100;
const PART_TWO_CUPS: u32 = 1_000_000;
const PART_TWO_MOVES: usize = 10_000_000;

/// The circle of cups, stored as a successor table so every move is O(1).
pub struct Cups {
    current: u32,
    /// `next[label]` is the label of the cup immediately clockwise of `label`.
    /// Index 0 is unused because labels start at 1.
    next: Vec<u32>,
}

impl Cups {
    /// Parses the starting labels, then pads the circle with consecutive
    /// labels up to `total` cups (no padding if `total` is not larger than
    /// the number of parsed labels).
    pub fn parse(input: &str, total: u32) -> Option<Self> {
        let labels: Vec<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<_>>()?;
        let first = *labels.first()?;
        let parsed = u32::try_from(labels.len()).ok()?;
        let max = total.max(parsed);

        let mut next = vec![0; max as usize + 1];
        let mut prev = first;
        for label in labels.iter().copied().skip(1).chain(parsed + 1..=max) {
            next[prev as usize] = label;
            prev = label;
        }
        next[prev as usize] = first;

        Some(Self {
            current: first,
            next,
        })
    }

    fn max_label(&self) -> u32 {
        (self.next.len() - 1) as u32
    }

    fn after(&self, label: u32) -> u32 {
        self.next[label as usize]
    }

    pub fn move_round(&mut self) {
        let max = self.max_label();
        let a = self.after(self.current);
        let b = self.after(a);
        let c = self.after(b);

        // Unlink the three picked-up cups.
        self.next[self.current as usize] = self.after(c);

        let below = |label: u32| if label == 1 { max } else { label - 1 };
        let mut destination = below(self.current);
        while destination == a || destination == b || destination == c {
            destination = below(destination);
        }

        // Splice them back in immediately clockwise of the destination.
        self.next[c as usize] = self.after(destination);
        self.next[destination as usize] = a;

        self.current = self.after(self.current);
    }

    pub fn play(&mut self, moves: usize) {
        for _ in 0..moves {
            self.move_round();
        }
    }

    /// Labels clockwise from cup 1, excluding cup 1 itself.
    pub fn labels_after_one(&self) -> impl Iterator<Item = u32> + '_ {
        std::iter::successors(Some(self.after(1)), |&label| Some(self.after(label)))
            .take_while(|&label| label != 1)
    }

    pub fn current_order(&self) -> String {
        self.labels_after_one()
            .map(|label| label.to_string())
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cups = Cups::parse(input, 0)?;
    cups.play(PART_ONE_MOVES);
    Some(cups.current_order())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cups = Cups::parse(input, PART_TWO_CUPS)?;
    cups.play(PART_TWO_MOVES);
    Some(
        cups.labels_after_one()
            .take(2)
            .map(|l| l as u64)
            .product::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_after_moves(moves: usize) -> String {
        let data = &advent_of_code::template::read_file("examples", DAY);
        let mut cups = Cups::parse(data, 0).unwrap();
        cups.play(moves);
        cups.current_order()
    }

    #[test]
    fn test_move_one() {
        assert_eq!("54673289", order_after_moves(1));
    }

    #[test]
    fn test_move_ten() {
        assert_eq!("92658374", order_after_moves(10));
    }

    #[test]
    fn test_move_hundred() {
        assert_eq!("67384529", order_after_moves(100));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("67384529"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(149245887792));
    }
}
