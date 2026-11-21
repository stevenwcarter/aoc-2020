advent_of_code::solution!(15);

/// Numbers below this are looked up directly in `last_seen`: they recur often,
/// so that part of the table stays in cache. Larger numbers go through the
/// `seen` bitset first.
const DIRECT_LIMIT: u32 = 1 << 16;

pub fn solve(input: &str, dest: u32) -> u32 {
    let size = dest as usize;
    // Turn on which each number was last spoken; 0 means never (turns start at 1).
    let mut last_seen = vec![0u32; size];
    // One bit per number: has it been spoken before? Small enough to stay in
    // cache, so a first-time number never has to wait on a `last_seen` read.
    let mut seen = vec![0u64; size.div_ceil(64)];

    let mut nums = input.trim().split(',').map(|s| s.parse::<u32>().unwrap());
    let mut turn: u32 = 1;
    let mut last = nums.next().unwrap();

    for n in nums {
        let i = last as usize;
        last_seen[i] = turn;
        seen[i >> 6] |= 1 << (i & 63);
        last = n;
        turn += 1;
    }

    while turn < dest {
        let i = last as usize;
        let next = if last < DIRECT_LIMIT {
            let ls = last_seen[i];
            if ls == 0 {
                0
            } else {
                turn - ls
            }
        } else {
            let word = &mut seen[i >> 6];
            let bit = 1u64 << (i & 63);
            if *word & bit == 0 {
                *word |= bit;
                0
            } else {
                turn - last_seen[i]
            }
        };
        last_seen[i] = turn;
        last = next;
        turn += 1;
    }

    last
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2020))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 30_000_000))
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("0,3,6"), Some(436));
        assert_eq!(part_one("1,3,2"), Some(1));
        assert_eq!(part_one("2,1,3"), Some(10));
        assert_eq!(part_one("1,2,3"), Some(27));
        assert_eq!(part_one("2,3,1"), Some(78));
        assert_eq!(part_one("3,2,1"), Some(438));
        assert_eq!(part_one("3,1,2"), Some(1836));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("0,3,6"), Some(175594));
        assert_eq!(part_two("1,3,2"), Some(2578));
        assert_eq!(part_two("2,1,3"), Some(3544142));
        assert_eq!(part_two("1,2,3"), Some(261214));
        assert_eq!(part_two("2,3,1"), Some(6895259));
        assert_eq!(part_two("3,2,1"), Some(18));
        assert_eq!(part_two("3,1,2"), Some(362));
    }
}
