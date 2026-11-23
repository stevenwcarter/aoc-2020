use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(21);

pub struct Record<'a> {
    pub ingredients: HashSet<&'a str>,
    pub allergens: Vec<&'a str>,
}

pub fn parse<'a>(input: &'a str) -> Vec<Record<'a>> {
    input
        .lines()
        .map(|l| {
            let (ingredients, allergens) = l.split_once(" (contains ").unwrap();

            let ingredients = ingredients.trim().split(" ").collect::<HashSet<&str>>();
            let allergens = allergens.strip_suffix(")").unwrap();
            let allergens = allergens.split(", ").collect::<Vec<&str>>();

            Record {
                ingredients,
                allergens,
            }
        })
        .collect()
}

/// For each allergen, the ingredients that appear in every food listing it.
fn candidates<'a>(records: &[Record<'a>]) -> HashMap<&'a str, HashSet<&'a str>> {
    records.iter().fold(HashMap::new(), |mut acc, record| {
        for &allergen in &record.allergens {
            acc.entry(allergen)
                .and_modify(|existing: &mut HashSet<&str>| {
                    existing.retain(|ing| record.ingredients.contains(ing));
                })
                .or_insert_with(|| record.ingredients.clone());
        }

        acc
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let records = parse(input);
    let candidates = candidates(&records);

    let possible_allergen_ingredients: HashSet<&str> = candidates
        .values()
        .flat_map(|set| set.iter().copied())
        .collect();

    Some(
        records
            .iter()
            .flat_map(|record| record.ingredients.iter())
            .filter(|ing| !possible_allergen_ingredients.contains(*ing))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let records = parse(input);
    let mut candidates = candidates(&records);
    let mut resolved: Vec<(&str, &str)> = Vec::with_capacity(candidates.len());

    // Repeatedly pin down an allergen with a single candidate, then strike
    // that ingredient from every other allergen's candidates.
    while let Some((allergen, ingredient)) = candidates.iter().find_map(|(&allergen, set)| {
        (set.len() == 1).then(|| (allergen, *set.iter().next().unwrap()))
    }) {
        candidates.remove(allergen);
        for set in candidates.values_mut() {
            set.remove(ingredient);
        }
        resolved.push((allergen, ingredient));
    }

    if !candidates.is_empty() {
        return None;
    }

    resolved.sort_unstable();
    Some(
        resolved
            .iter()
            .map(|&(_, ingredient)| ingredient)
            .collect::<Vec<_>>()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("mxmxvkd,sqjhc,fvjkl"));
    }
}
