advent_of_code::solution!(18);
use std::iter::Peekable;

fn solve(input: &str) -> i64 {
    let mut chars = input.chars().filter(|c| !c.is_whitespace()).peekable();
    eval_expr(&mut chars)
}
fn solve2(input: &str) -> i64 {
    let mut chars = input.chars().filter(|c| !c.is_whitespace()).peekable();
    eval_expr2(&mut chars)
}

fn eval_expr<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> i64 {
    let mut acc = eval_term(chars);

    while let Some(&op) = chars.peek() {
        match op {
            '+' => {
                chars.next();
                acc += eval_term(chars);
            }
            '*' => {
                chars.next();
                acc *= eval_term(chars);
            }
            ')' => {
                chars.next(); // Consume the closing parenthesis
                break;
            }
            _ => panic!("Unexpected character: {}", op),
        }
    }

    acc
}
/// Part 2: an expression is a product of sums (`+` binds tighter than `*`).
fn eval_expr2<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> i64 {
    let mut acc = eval_sum2(chars);

    while let Some(&op) = chars.peek() {
        match op {
            '*' => {
                chars.next();
                acc *= eval_sum2(chars);
            }
            ')' => {
                chars.next(); // Consume the closing parenthesis
                break;
            }
            _ => panic!("Unexpected character: {}", op),
        }
    }

    acc
}

fn eval_sum2<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> i64 {
    let mut acc = eval_term2(chars);

    while chars.next_if_eq(&'+').is_some() {
        acc += eval_term2(chars);
    }

    acc
}

fn eval_term2<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> i64 {
    let next = chars.next();
    if next == Some('(') {
        return eval_expr2(chars);
    }
    let Some(digit) = next.and_then(|c| c.to_digit(10)) else {
        panic!("Expected '(' or digit, found {:?}", next);
    };
    parse_number(digit, chars)
}

fn eval_term<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> i64 {
    match chars.next() {
        Some('(') => eval_expr(chars),
        Some(c) if c.is_ascii_digit() => parse_number(c.to_digit(10).unwrap(), chars),
        other => panic!("Expected '(' or digit, found {:?}", other),
    }
}

/// Parses the rest of a decimal number whose first digit has already been consumed.
fn parse_number<I: Iterator<Item = char>>(first_digit: u32, chars: &mut Peekable<I>) -> i64 {
    let mut num = i64::from(first_digit);
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        chars.next();
        num = num * 10 + i64::from(digit);
    }
    num
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| solve(l)).sum::<i64>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(solve2).sum::<i64>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_0() {
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }
    #[test]
    fn test_expr_1() {
        assert_eq!(solve("2 * 3 + (5 * 4)"), 26);
    }
    #[test]
    fn test_expr_2() {
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }
    #[test]
    fn test_expr_3() {
        assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    }
    #[test]
    fn test_expr_4() {
        assert_eq!(
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_expr2_0() {
        assert_eq!(solve2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    }
    #[test]
    fn test_expr2_1() {
        assert_eq!(solve2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }
    #[test]
    fn test_expr2_2() {
        assert_eq!(solve2("2 * 3 + (5 * 4)"), 46);
    }
    #[test]
    fn test_expr2_3() {
        assert_eq!(solve2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }
    #[test]
    fn test_expr2_4() {
        assert_eq!(solve2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    }
    #[test]
    fn test_expr2_5() {
        assert_eq!(
            solve2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
