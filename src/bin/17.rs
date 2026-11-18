use hashbrown::HashSet;

advent_of_code::solution!(17);

fn return_neighbors_for_coord(x: isize, y: isize, z: isize) -> Vec<(isize, isize, isize)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x - 1, y - 1, z),
        (x - 1, y + 1, z),
        (x + 1, y - 1, z),
        (x + 1, y + 1, z),
        (x - 1, y, z - 1),
        (x - 1, y - 1, z - 1),
        (x - 1, y + 1, z - 1),
        (x + 1, y, z - 1),
        (x + 1, y - 1, z - 1),
        (x + 1, y + 1, z - 1),
        (x, y - 1, z - 1),
        (x, y, z - 1),
        (x, y + 1, z - 1),
        (x - 1, y, z + 1),
        (x - 1, y - 1, z + 1),
        (x - 1, y + 1, z + 1),
        (x + 1, y, z + 1),
        (x + 1, y - 1, z + 1),
        (x + 1, y + 1, z + 1),
        (x, y - 1, z + 1),
        (x, y, z + 1),
        (x, y + 1, z + 1),
    ]
}
fn return_neighbors_for_coord_part2(
    x: isize,
    y: isize,
    z: isize,
    t: isize,
) -> Vec<(isize, isize, isize, isize)> {
    vec![
        (x - 1, y, z, t - 1),
        (x + 1, y, z, t - 1),
        (x, y - 1, z, t - 1),
        (x, y + 1, z, t - 1),
        (x - 1, y - 1, z, t - 1),
        (x - 1, y + 1, z, t - 1),
        (x + 1, y - 1, z, t - 1),
        (x + 1, y + 1, z, t - 1),
        (x - 1, y, z - 1, t - 1),
        (x - 1, y - 1, z - 1, t - 1),
        (x - 1, y + 1, z - 1, t - 1),
        (x + 1, y, z - 1, t - 1),
        (x + 1, y - 1, z - 1, t - 1),
        (x + 1, y + 1, z - 1, t - 1),
        (x, y - 1, z - 1, t - 1),
        (x, y, z - 1, t - 1),
        (x, y + 1, z - 1, t - 1),
        (x - 1, y, z + 1, t - 1),
        (x - 1, y - 1, z + 1, t - 1),
        (x - 1, y + 1, z + 1, t - 1),
        (x + 1, y, z + 1, t - 1),
        (x + 1, y - 1, z + 1, t - 1),
        (x + 1, y + 1, z + 1, t - 1),
        (x, y - 1, z + 1, t - 1),
        (x, y, z + 1, t - 1),
        (x, y + 1, z + 1, t - 1),
        (x, y, z, t - 1),
        (x - 1, y, z, t + 1),
        (x + 1, y, z, t + 1),
        (x, y - 1, z, t + 1),
        (x, y + 1, z, t + 1),
        (x - 1, y - 1, z, t + 1),
        (x - 1, y + 1, z, t + 1),
        (x + 1, y - 1, z, t + 1),
        (x + 1, y + 1, z, t + 1),
        (x - 1, y, z - 1, t + 1),
        (x - 1, y - 1, z - 1, t + 1),
        (x - 1, y + 1, z - 1, t + 1),
        (x + 1, y, z - 1, t + 1),
        (x + 1, y - 1, z - 1, t + 1),
        (x + 1, y + 1, z - 1, t + 1),
        (x, y - 1, z - 1, t + 1),
        (x, y, z - 1, t + 1),
        (x, y + 1, z - 1, t + 1),
        (x - 1, y, z + 1, t + 1),
        (x - 1, y - 1, z + 1, t + 1),
        (x - 1, y + 1, z + 1, t + 1),
        (x + 1, y, z + 1, t + 1),
        (x + 1, y - 1, z + 1, t + 1),
        (x + 1, y + 1, z + 1, t + 1),
        (x, y - 1, z + 1, t + 1),
        (x, y, z + 1, t + 1),
        (x, y + 1, z + 1, t + 1),
        (x, y, z, t + 1),
        (x - 1, y, z, t),
        (x + 1, y, z, t),
        (x, y - 1, z, t),
        (x, y + 1, z, t),
        (x - 1, y - 1, z, t),
        (x - 1, y + 1, z, t),
        (x + 1, y - 1, z, t),
        (x + 1, y + 1, z, t),
        (x - 1, y, z - 1, t),
        (x - 1, y - 1, z - 1, t),
        (x - 1, y + 1, z - 1, t),
        (x + 1, y, z - 1, t),
        (x + 1, y - 1, z - 1, t),
        (x + 1, y + 1, z - 1, t),
        (x, y - 1, z - 1, t),
        (x, y, z - 1, t),
        (x, y + 1, z - 1, t),
        (x - 1, y, z + 1, t),
        (x - 1, y - 1, z + 1, t),
        (x - 1, y + 1, z + 1, t),
        (x + 1, y, z + 1, t),
        (x + 1, y - 1, z + 1, t),
        (x + 1, y + 1, z + 1, t),
        (x, y - 1, z + 1, t),
        (x, y, z + 1, t),
        (x, y + 1, z + 1, t),
    ]
}

#[derive(Debug, Clone)]
pub struct Region(HashSet<(isize, isize, isize)>);

#[derive(Debug, Clone)]
pub struct Region2(HashSet<(isize, isize, isize, isize)>);

impl Region {
    pub fn parse(input: &str) -> Self {
        let mut region: HashSet<(isize, isize, isize)> = HashSet::new();

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    region.insert((x as isize, y as isize, 0));
                });
        });

        Self(region)
    }

    pub fn count_active_neighbors_at_coord(&self, x: isize, y: isize, z: isize) -> u8 {
        let neighbors = return_neighbors_for_coord(x, y, z);

        neighbors.iter().filter(|n| self.0.contains(*n)).count() as u8
    }

    pub fn process(&mut self) {
        let mut new_region = self.clone();

        self.0.iter().for_each(|c| {
            let neighbors = return_neighbors_for_coord(c.0, c.1, c.2);

            neighbors.iter().chain(std::iter::once(c)).for_each(|n| {
                let is_active = self.0.contains(n);
                match (
                    is_active,
                    self.count_active_neighbors_at_coord(n.0, n.1, n.2),
                ) {
                    (true, 0..2 | 4..) => {
                        new_region.0.remove(n);
                    }
                    (false, 3) => {
                        new_region.0.insert(*n);
                    }
                    _ => {}
                }
            })
        });

        self.0 = new_region.0;
    }
}
impl Region2 {
    pub fn parse(input: &str) -> Self {
        let mut region: HashSet<(isize, isize, isize, isize)> = HashSet::new();

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    region.insert((x as isize, y as isize, 0, 0));
                });
        });

        Self(region)
    }

    pub fn count_active_neighbors_at_coord(&self, c: (isize, isize, isize, isize)) -> u8 {
        let neighbors = return_neighbors_for_coord_part2(c.0, c.1, c.2, c.3);

        neighbors.iter().filter(|n| self.0.contains(*n)).count() as u8
    }

    pub fn process(&mut self) {
        let mut new_region = self.clone();

        self.0.iter().for_each(|c| {
            let neighbors = return_neighbors_for_coord_part2(c.0, c.1, c.2, c.3);

            neighbors.iter().chain(std::iter::once(c)).for_each(|n| {
                let is_active = self.0.contains(n);
                match (is_active, self.count_active_neighbors_at_coord(*n)) {
                    (true, 0..2 | 4..) => {
                        new_region.0.remove(n);
                    }
                    (false, 3) => {
                        new_region.0.insert(*n);
                    }
                    _ => {}
                }
            })
        });

        self.0 = new_region.0;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut region = Region::parse(input);

    (0..6).for_each(|_| {
        region.process();
    });

    Some(region.0.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut region = Region2::parse(input);

    (0..6).for_each(|_| {
        region.process();
    });

    Some(region.0.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(848));
    }
}
