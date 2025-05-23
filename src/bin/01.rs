use hashbrown::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let data: HashSet<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let result = find_number_adds_to_total(&data, 2020);

    result.map(|(x, y)| x * y)
}

pub fn find_number_adds_to_total(data: &HashSet<i64>, total: i64) -> Option<(i64, i64)> {
    for x in data.iter() {
        if data.contains(&(total - x)) {
            return Some((*x, total - x));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<i64> {
    let data: HashSet<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    for x in data.iter() {
        match find_number_adds_to_total(&data, 2020 - x) {
            Some((y, z)) => return Some(*x * y * z),
            None => continue,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
