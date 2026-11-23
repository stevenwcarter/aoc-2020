advent_of_code::solution!(25);

pub fn transform(subject_number: u64, loop_size: u32) -> u64 {
    (0..loop_size).fold(1, |acc, _| loop_step(acc, subject_number))
}

pub fn loop_step(original_number: u64, subject_number: u64) -> u64 {
    (original_number * subject_number) % 20201227
}

pub fn find_loop_size(src: u64) -> u32 {
    let mut result = 1;
    let mut loop_size = 0;
    while result != src {
        loop_size += 1;

        result = loop_step(result, 7);
    }

    loop_size
}
pub fn part_one(input: &str) -> Option<u64> {
    let (card, door) = input
        .trim()
        .split_once("\n")
        .expect("Input should be two number separated by newline");

    let card = str::parse::<u64>(card).expect("Card should be a number");
    let door = str::parse::<u64>(door).expect("Card should be a number");

    let door_loop_size = find_loop_size(door);

    Some(transform(card, door_loop_size))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(String::from("none"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let result = transform(7, 8);
        assert_eq!(result, 5764801);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14897079));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("none"));
    }
}
