advent_of_code::solution!(5);

pub fn get_seat_id(seat: &str) -> (u64, u64, u64) {
    let row = seat[..7]
        .chars()
        .fold(0, |acc, c| (acc << 1) | if c == 'B' { 1 } else { 0 });
    let col = seat[7..]
        .chars()
        .fold(0, |acc, c| (acc << 1) | if c == 'R' { 1 } else { 0 });
    (row, col, row * 8 + col)
}

pub fn part_one(input: &str) -> Option<u64> {
    let max_seat_id = input
        .lines()
        .map(get_seat_id)
        .map(|(_, _, id)| id)
        .max()
        .unwrap_or(0);

    Some(max_seat_id)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut seat_ids = input
        .lines()
        .map(get_seat_id)
        .map(|(_, _, id)| id)
        .collect::<Vec<u64>>();

    seat_ids.sort_unstable();

    let mut missing_seat_id = None;
    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i - 1] > 1 {
            missing_seat_id = Some(seat_ids[i] - 1);
            break;
        }
    }

    missing_seat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_id() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), (70, 7, 567));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(566));
    }
}
