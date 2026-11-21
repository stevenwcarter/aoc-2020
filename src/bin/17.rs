use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(17);

type Cell<const N: usize> = [i8; N];

fn neighbors<const N: usize>(cell: Cell<N>) -> impl Iterator<Item = Cell<N>> {
    let total = 3usize.pow(N as u32);
    (0..total)
        .filter(move |&i| i != total / 2) // middle index = all-zero offset
        .map(move |mut i| {
            let mut out = cell;
            for c in &mut out {
                *c += (i % 3) as i8 - 1;
                i /= 3;
            }
            out
        })
}

fn parse<const N: usize>(input: &str) -> HashSet<Cell<N>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| {
                    let mut cell = [0; N];
                    cell[0] = x as i8;
                    cell[1] = y as i8;
                    cell
                })
        })
        .collect()
}

fn step<const N: usize>(active: &HashSet<Cell<N>>) -> HashSet<Cell<N>> {
    let mut counts: HashMap<Cell<N>, u8> = HashMap::new();
    for &cell in active {
        for n in neighbors(cell) {
            *counts.entry(n).or_default() += 1;
        }
    }
    counts
        .into_iter()
        .filter(|&(cell, n)| n == 3 || (n == 2 && active.contains(&cell)))
        .map(|(cell, _)| cell)
        .collect()
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut active = parse::<N>(input);
    for _ in 0..6 {
        active = step(&active);
    }
    active.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve::<3>(input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve::<4>(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(848));
    }
}
