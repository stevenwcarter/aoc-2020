advent_of_code::solution!(13);

pub fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(
    x: T,
    y: T,
) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();
    let earliest_time = lines[0].parse::<u64>().ok()?;
    let bus_ids: Vec<u64> = lines[1]
        .split(',')
        .filter_map(|id| id.parse::<u64>().ok())
        .collect();

    let (mut earliest_found, mut earliest_bus_id): (u64, u64) = (earliest_time * 2, 0);

    bus_ids.iter().for_each(|&id| {
        let (quotient, remainder) = div_rem(earliest_time, id);
        if remainder == 0 {
            earliest_found = earliest_time;
            earliest_bus_id = id;
        } else {
            let possible_time = (quotient + 1) * id;
            if possible_time < earliest_found {
                earliest_found = possible_time;
                earliest_bus_id = id;
            }
        }
    });

    Some(earliest_bus_id * (earliest_found - earliest_time))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();
    let bus_ids: Vec<Option<u64>> = lines[1]
        .split(',')
        .map(|id| id.parse::<u64>().ok())
        .collect();

    let mut time: u64 = 0;
    let mut step: u64 = 1;
    for (offset, &bus_id) in bus_ids.iter().enumerate() {
        if let Some(bus_id) = bus_id {
            while (time + offset as u64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
    }
    Some(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1068781));
    }
}
