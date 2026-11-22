use hashbrown::HashMap;

advent_of_code::solution!(19);

/// Part 2 replacements that make rules 8 and 11 loop back on themselves.
const LOOPING_RULES: [&str; 2] = ["8: 42 | 42 8", "11: 42 31 | 42 11 31"];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RuleId(u32);

pub enum RuleType {
    Sequence(Vec<RuleId>),
    Conditional((Vec<RuleId>, Vec<RuleId>)),
    A,
    B,
}

pub struct Rules {
    rules: HashMap<RuleId, RuleType>,
}

fn parse_rule_ids(seq: &str) -> Vec<RuleId> {
    seq.split_whitespace()
        .map(|v| RuleId(v.parse().expect("should be a number")))
        .collect()
}

fn parse_sequence(seq: &str) -> RuleType {
    if let Some((left, right)) = seq.split_once('|') {
        return RuleType::Conditional((parse_rule_ids(left), parse_rule_ids(right)));
    }

    match seq.trim() {
        r#""a""# => RuleType::A,
        r#""b""# => RuleType::B,
        seq => RuleType::Sequence(parse_rule_ids(seq)),
    }
}

fn parse_line(line: &str) -> (RuleId, RuleType) {
    let (rule_id, rules) = line
        .trim()
        .split_once(':')
        .expect("should be 0: 2 3 4 format");
    let rule_id = rule_id.parse().expect("should be numerical");

    (RuleId(rule_id), parse_sequence(rules))
}

/// The remainder after `c` matches the first byte of `input`, if it does.
fn match_char(c: u8, input: &[u8]) -> Vec<&[u8]> {
    match input.split_first() {
        Some((first, rest)) if *first == c => vec![rest],
        _ => Vec::new(),
    }
}

impl Rules {
    pub fn parse(input: &str) -> Self {
        Self {
            rules: input.lines().map(parse_line).collect(),
        }
    }

    /// Replaces rules 8 and 11 with their looping part 2 definitions.
    pub fn with_looping_rules(mut self) -> Self {
        self.rules.extend(LOOPING_RULES.map(parse_line));
        self
    }

    /// All possible remainders after `id` matches a prefix of `input`.
    ///
    /// Returning every remainder (rather than the first) is what lets the
    /// looping rules backtrack; recursion terminates because each loop
    /// consumes at least one byte before recursing.
    fn match_rule<'a>(&self, id: RuleId, input: &'a [u8]) -> Vec<&'a [u8]> {
        match &self.rules[&id] {
            RuleType::A => match_char(b'a', input),
            RuleType::B => match_char(b'b', input),
            RuleType::Sequence(seq) => self.match_seq(seq, input),
            RuleType::Conditional((left, right)) => {
                let mut rems = self.match_seq(left, input);
                rems.extend(self.match_seq(right, input));
                rems
            }
        }
    }

    /// All possible remainders after every rule in `seq` matches in order.
    fn match_seq<'a>(&self, seq: &[RuleId], input: &'a [u8]) -> Vec<&'a [u8]> {
        seq.iter().fold(vec![input], |rems, id| {
            rems.into_iter()
                .flat_map(|rem| self.match_rule(*id, rem))
                .collect()
        })
    }

    /// Whether `id` matches the whole of `input`.
    pub fn matches(&self, id: RuleId, input: &str) -> bool {
        self.match_rule(id, input.as_bytes())
            .iter()
            .any(|rem| rem.is_empty())
    }
}

fn split_input(input: &str) -> (Rules, &str) {
    let (rules, messages) = input
        .split_once("\n\n")
        .expect("should be split by double newline");

    (Rules::parse(rules), messages)
}

fn count_matches(rules: &Rules, messages: &str) -> usize {
    messages
        .lines()
        .filter(|l| rules.matches(RuleId(0), l))
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, messages) = split_input(input);

    Some(count_matches(&rules, messages))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, messages) = split_input(input);

    Some(count_matches(&rules.with_looping_rules(), messages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(12));
    }
}
