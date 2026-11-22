advent_of_code::solution!(11);

use std::mem;

/// An occupied seat empties once this many adjacent seats are occupied.
const ADJACENT_TOLERANCE: u8 = 4;
/// An occupied seat empties once this many visible seats are occupied.
const VISIBLE_TOLERANCE: u8 = 5;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// Index of a seat in the compact, floor-free seat list.
type SeatId = u32;

/// Which seats count as neighbours of a seat.
#[derive(Clone, Copy)]
enum NeighbourRule {
    /// The seats in the eight surrounding cells.
    Adjacent,
    /// The first seat seen in each of the eight directions, looking past floor.
    Visible,
}

/// The neighbours of one seat. There are at most eight, so they are stored
/// inline rather than in a per-seat heap allocation.
#[derive(Clone, Copy, Default)]
struct Neighbours {
    seats: [SeatId; DIRECTIONS.len()],
    len: usize,
}

impl Neighbours {
    fn as_slice(&self) -> &[SeatId] {
        &self.seats[..self.len]
    }
}

impl FromIterator<SeatId> for Neighbours {
    fn from_iter<I: IntoIterator<Item = SeatId>>(iter: I) -> Self {
        let mut neighbours = Self::default();
        for seat in iter {
            neighbours.seats[neighbours.len] = seat;
            neighbours.len += 1;
        }
        neighbours
    }
}

/// The waiting area: where the seats are, and which start out occupied.
struct Layout {
    width: usize,
    height: usize,
    /// One entry per grid cell in row-major order; `None` is floor.
    seat_at: Vec<Option<SeatId>>,
    /// Initial occupancy, indexed by `SeatId`.
    occupied: Vec<bool>,
}

impl Layout {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().map(str::trim).collect();
        let width = lines.first().map_or(0, |line| line.len());

        let mut occupied = Vec::new();
        let mut seat_at = Vec::with_capacity(width * lines.len());
        for line in &lines {
            assert_eq!(line.len(), width, "all rows must have the same width");
            for byte in line.bytes() {
                let seat = match byte {
                    b'.' => None,
                    b'L' => Some(false),
                    b'#' => Some(true),
                    other => panic!("Invalid {:?} found while parsing", char::from(other)),
                };
                seat_at.push(seat.map(|is_occupied| {
                    let id = SeatId::try_from(occupied.len()).expect("seat count fits in SeatId");
                    occupied.push(is_occupied);
                    id
                }));
            }
        }

        Self {
            width,
            height: lines.len(),
            seat_at,
            occupied,
        }
    }

    /// Moves one cell in `direction`, or returns `None` when that leaves the grid.
    fn step(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
        let x = x.checked_add_signed(dx).filter(|&x| x < self.width)?;
        let y = y.checked_add_signed(dy).filter(|&y| y < self.height)?;
        Some((x, y))
    }

    /// The neighbouring seat of the cell at `position` in `direction`, if any.
    fn neighbour_towards(
        &self,
        mut position: (usize, usize),
        direction: (isize, isize),
        rule: NeighbourRule,
    ) -> Option<SeatId> {
        loop {
            position = self.step(position, direction)?;
            let (x, y) = position;
            match (self.seat_at[y * self.width + x], rule) {
                (Some(seat), _) => return Some(seat),
                (None, NeighbourRule::Adjacent) => return None,
                (None, NeighbourRule::Visible) => {}
            }
        }
    }

    /// The neighbours of every seat, indexed by `SeatId`. Floor never changes,
    /// so this is computed once and holds for the whole simulation.
    fn neighbours(&self, rule: NeighbourRule) -> Vec<Neighbours> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.seat_at[y * self.width + x].is_some())
            .map(|position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|&direction| self.neighbour_towards(position, direction, rule))
                    .collect()
            })
            .collect()
    }
}

/// Applies the seating rules until nobody moves, then counts the occupied seats.
fn occupied_when_stable(
    mut occupied: Vec<bool>,
    neighbours: &[Neighbours],
    tolerance: u8,
) -> usize {
    let seat_count = occupied.len();

    // Kept up to date as seats flip, rather than recounted every generation.
    let mut occupied_neighbours = vec![0u8; seat_count];
    for (neighbours, &is_occupied) in neighbours.iter().zip(&occupied) {
        if is_occupied {
            for &neighbour in neighbours.as_slice() {
                occupied_neighbours[neighbour as usize] += 1;
            }
        }
    }

    // A seat's fate depends only on its own state and its occupied-neighbour
    // count, so after the first generation only the neighbours of seats that
    // just flipped can flip next. A seat that flipped while its neighbours
    // stayed put is already settled: its count is unchanged, and that count
    // is what put it in its new state.
    let mut needs_check = vec![true; seat_count];
    let mut flips = Vec::new();

    loop {
        // Decide every flip from the counts as they stand before applying any.
        flips.clear();
        for seat in 0..seat_count {
            if !mem::take(&mut needs_check[seat]) {
                continue;
            }
            let must_flip = if occupied[seat] {
                occupied_neighbours[seat] >= tolerance
            } else {
                occupied_neighbours[seat] == 0
            };
            if must_flip {
                flips.push(seat);
            }
        }
        if flips.is_empty() {
            return occupied.iter().filter(|&&is_occupied| is_occupied).count();
        }

        for &seat in &flips {
            let now_occupied = !occupied[seat];
            occupied[seat] = now_occupied;
            for &neighbour in neighbours[seat].as_slice() {
                let neighbour = neighbour as usize;
                if now_occupied {
                    occupied_neighbours[neighbour] += 1;
                } else {
                    occupied_neighbours[neighbour] -= 1;
                }
                needs_check[neighbour] = true;
            }
        }
    }
}

fn solve(input: &str, rule: NeighbourRule, tolerance: u8) -> usize {
    let layout = Layout::parse(input);
    let neighbours = layout.neighbours(rule);
    occupied_when_stable(layout.occupied, &neighbours, tolerance)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, NeighbourRule::Adjacent, ADJACENT_TOLERANCE))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, NeighbourRule::Visible, VISIBLE_TOLERANCE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
