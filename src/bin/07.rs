use hashbrown::HashSet;

advent_of_code::solution!(7);

pub struct BagRule {
    color: String,
    contains: Vec<(u64, String)>,
}

impl BagRule {
    pub fn new(color: String, contains: Vec<(u64, String)>) -> Self {
        Self { color, contains }
    }

    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" bags contain ").collect();
        let color = parts[0].to_string();
        let contains = if parts[1] == "no other bags." {
            vec![]
        } else {
            parts[1]
                .trim_end_matches('.')
                .split(", ")
                .map(|s| {
                    let s = s.trim_end_matches(" bags").trim_end_matches(" bag");
                    let count = s.split_whitespace().next().unwrap().parse::<u64>().unwrap();
                    let color = s
                        .split_whitespace()
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .join(" ");
                    (count, color)
                })
                .collect()
        };
        Self::new(color, contains)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let bag_rules = input.lines().map(BagRule::parse).collect::<Vec<BagRule>>();

    let mut queue = vec!["shiny gold"];
    let mut visited = HashSet::new();
    while let Some(color) = queue.pop() {
        if visited.contains(color) {
            continue;
        }
        visited.insert(color);
        for rule in &bag_rules {
            if rule.contains.iter().any(|(_, c)| c == color) {
                queue.push(&rule.color);
            }
        }
    }

    // Remove the "shiny gold" bag itself from the count
    Some(visited.len() - 1)
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
                for (c, inner_color) in &rule.contains {
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
