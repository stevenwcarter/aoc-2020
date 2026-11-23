advent_of_code::solution!(24);
use std::collections::{HashMap, HashSet};

const DIRS: [(i32, i32); 6] = [
    (1, 0),  // e
    (-1, 0), // w
    (0, 1),  // ne
    (0, -1), // sw
    (-1, 1), // nw
    (1, -1), // se
];

fn parse_line(line: &str) -> (i32, i32) {
    let mut q = 0;
    let mut r = 0;
    let mut bytes = line.bytes();

    while let Some(b) = bytes.next() {
        let (dq, dr) = match b {
            b'e' => (1, 0),
            b'w' => (-1, 0),
            b'n' => match bytes.next() {
                Some(b'e') => (0, 1),
                Some(b'w') => (-1, 1),
                _ => unreachable!("invalid direction"),
            },
            b's' => match bytes.next() {
                Some(b'e') => (1, -1),
                Some(b'w') => (0, -1),
                _ => unreachable!("invalid direction"),
            },
            _ => continue,
        };
        q += dq;
        r += dr;
    }

    (q, r)
}

fn parse_initial_black_tiles(input: &str) -> HashSet<(i32, i32)> {
    let mut black = HashSet::new();
    for line in input.lines() {
        let tile = parse_line(line);
        // HashSet::remove returns true if the tile was present
        if !black.remove(&tile) {
            black.insert(tile);
        }
    }
    black
}

fn step_day(black: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut neighbor_counts: HashMap<(i32, i32), u8> = HashMap::new();

    for &(q, r) in black {
        for &(dq, dr) in &DIRS {
            *neighbor_counts.entry((q + dq, r + dr)).or_insert(0) += 1;
        }
    }

    neighbor_counts
        .into_iter()
        .filter_map(|(tile, count)| {
            if count == 2 || (count == 1 && black.contains(&tile)) {
                Some(tile)
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let black = parse_initial_black_tiles(input);
    Some(black.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut black = parse_initial_black_tiles(input);

    for _ in 0..100 {
        black = step_day(&black);
    }
    Some(black.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2208));
    }
}
