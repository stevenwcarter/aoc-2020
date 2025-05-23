advent_of_code::solution!(4);

pub struct Entry<'a> {
    pub lines: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    pub fn parse(input: &'a str) -> Vec<Self> {
        let mut entries = Vec::new();
        let mut lines = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                entries.push(Entry { lines });
                lines = Vec::new();
                continue;
            } else {
                lines.push(line);
            }
        }
        entries.push(Entry { lines });

        entries
    }

    pub fn check_has_field(&self, field: &str) -> bool {
        for line in &self.lines {
            if line.contains(field) {
                return true;
            }
        }
        false
    }

    pub fn has_all_fields(&self, required_fields: &[&str]) -> bool {
        for field in required_fields {
            if !self.check_has_field(field) {
                return false;
            }
        }
        true
    }

    pub fn check_valid_byr(&self) -> bool {
        for line in &self.lines {
            if line.contains("byr:") {
                let index = line.find("byr:").unwrap();
                let year_str = &line[index + 4..index + 8];
                if let Ok(year) = year_str.trim().parse::<u32>() {
                    return (1920..=2002).contains(&year);
                }
            }
        }
        false
    }

    pub fn check_valid_iyr(&self) -> bool {
        for line in &self.lines {
            if line.contains("iyr:") {
                let index = line.find("iyr:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(year_str) = parts.first() {
                    if let Ok(year) = year_str.trim().parse::<u32>() {
                        return (2010..=2020).contains(&year);
                    }
                }
            }
        }
        false
    }

    pub fn check_valid_eyr(&self) -> bool {
        for line in &self.lines {
            if line.contains("eyr:") {
                let index = line.find("eyr:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(year_str) = parts.first() {
                    if let Ok(year) = year_str.trim().parse::<u32>() {
                        return (2020..=2030).contains(&year);
                    }
                }
            }
        }
        false
    }

    pub fn check_valid_hgt(&self) -> bool {
        for line in &self.lines {
            if line.contains("hgt:") {
                let index = line.find("hgt:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(height_str) = parts.first() {
                    let height_str = height_str.trim();
                    if height_str.ends_with("cm") {
                        let height = &height_str[..height_str.len() - 2];
                        if let Ok(height) = height.parse::<u32>() {
                            return (150..=193).contains(&height);
                        }
                    } else if height_str.ends_with("in") {
                        let height = &height_str[..height_str.len() - 2];
                        if let Ok(height) = height.parse::<u32>() {
                            return (59..=76).contains(&height);
                        }
                    }
                }
            }
        }
        false
    }

    pub fn check_valid_hcl(&self) -> bool {
        for line in &self.lines {
            if line.contains("hcl:") {
                let index = line.find("hcl:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(color_str) = parts.first() {
                    let color_str = color_str.trim();
                    if color_str.len() == 7 && color_str.starts_with('#') {
                        return color_str[1..].chars().all(|c| c.is_ascii_hexdigit());
                    }
                }
            }
        }
        false
    }

    pub fn check_valid_ecl(&self) -> bool {
        for line in &self.lines {
            if line.contains("ecl:") {
                let index = line.find("ecl:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(color_str) = parts.first() {
                    let color_str = color_str.trim();
                    return matches!(
                        color_str,
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    );
                }
            }
        }
        false
    }

    pub fn check_valid_pid(&self) -> bool {
        for line in &self.lines {
            if line.contains("pid:") {
                let index = line.find("pid:").unwrap();
                let parts: Vec<&str> = line[index + 4..].split(' ').collect();
                if let Some(pid_str) = parts.first() {
                    let pid_str = pid_str.trim();
                    return pid_str.len() == 9 && pid_str.chars().all(|c| c.is_ascii_digit());
                }
            }
        }
        false
    }

    pub fn is_valid(&self) -> bool {
        self.check_valid_byr()
            && self.check_valid_iyr()
            && self.check_valid_eyr()
            && self.check_valid_hgt()
            && self.check_valid_hcl()
            && self.check_valid_ecl()
            && self.check_valid_pid()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    let mut entries: Vec<Entry> = Entry::parse(input);

    entries.retain(|entry| entry.has_all_fields(&required_fields));

    Some(entries.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut entries: Vec<Entry> = Entry::parse(input);

    entries.retain(|entry| entry.is_valid());

    Some(entries.len())
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
        assert_eq!(result, Some(4));
    }
}
