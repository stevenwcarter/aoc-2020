advent_of_code::solution!(15);

pub fn solve(input: &str, dest: u32) -> u32 {
    let mut last_seen = vec![0u32; dest as usize];

    let mut nums = input.trim().split(',').map(|s| s.parse::<u32>().unwrap());
    let mut turn: u32 = 1;
    let first = nums.next().unwrap();
    let mut last = first;

    for n in nums {
        last_seen[last as usize] = turn;
        last = n;
        turn += 1;
    }

    while turn < dest {
        let ls = *last_seen.get(last as usize).unwrap();
        let next = if ls == 0 { 0 } else { turn - ls };
        *last_seen.get_mut(last as usize).unwrap() = turn;
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
