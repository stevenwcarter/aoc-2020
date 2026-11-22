use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(7);

pub struct BagRule<'a> {
    color: &'a str,
    contains: Vec<(u64, &'a str)>,
}

impl<'a> BagRule<'a> {
    pub fn new(color: &'a str, contains: Vec<(u64, &'a str)>) -> Self {
        Self { color, contains }
    }

    pub fn parse(input: &'a str) -> Self {
        let (color, contents) = input.split_once(" bags contain ").unwrap();
        let contains = if contents == "no other bags." {
            vec![]
        } else {
            contents
                .trim_end_matches('.')
                .split(", ")
                .map(|s| {
                    let s = s.trim_end_matches(" bags").trim_end_matches(" bag");
                    let (count, color) = s.split_once(' ').unwrap();
                    (count.parse::<u64>().unwrap(), color)
                })
                .collect()
        };
        Self::new(color, contains)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let rules: Vec<BagRule> = input.lines().map(BagRule::parse).collect();

    let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in &rules {
        for &(_, inner) in &rule.contains {
            parents.entry(inner).or_default().push(rule.color);
        }
    }

    let mut stack = vec!["shiny gold"];
    let mut visited = HashSet::new();
    while let Some(color) = stack.pop() {
        for &outer in parents.get(color).into_iter().flatten() {
            if visited.insert(outer) {
                stack.push(outer);
            }
        }
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u64> {
    let bag_rules = input.lines().map(BagRule::parse).collect::<Vec<BagRule>>();

    let mut queue = vec![("shiny gold", 1)];
    let mut total_bags = 0;
    while let Some((color, count)) = queue.pop() {
        bag_rules
            .iter()
            .filter(|rule| rule.color == color)
            .for_each(|rule| {
                for &(c, inner_color) in &rule.contains {
                    total_bags += c * count;
                    queue.push((inner_color, c * count));
                }
            });
    }
    Some(total_bags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }
}
