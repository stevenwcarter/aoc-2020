use std::{cmp, iter};

use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(20);

/// Square bitmap, `true` for `#`.
type Grid = Vec<Vec<bool>>;
type Edge = Vec<bool>;

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

struct Tile {
    id: u64,
    pixels: Grid,
}

/// The four borders of a grid; rows read left to right, columns top to bottom.
struct Edges {
    top: Edge,
    bottom: Edge,
    left: Edge,
    right: Edge,
}

impl Edges {
    fn into_array(self) -> [Edge; 4] {
        [self.top, self.bottom, self.left, self.right]
    }
}

fn parse_grid<'a>(lines: impl Iterator<Item = &'a str>) -> Grid {
    lines
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect()
}

fn parse_tile(block: &str) -> Option<Tile> {
    let mut lines = block.lines();
    let id = lines
        .next()?
        .strip_prefix("Tile ")?
        .strip_suffix(':')?
        .parse()
        .ok()?;
    let pixels = parse_grid(lines);
    (!pixels.is_empty()).then_some(Tile { id, pixels })
}

fn parse(input: &str) -> Option<Vec<Tile>> {
    input
        .split("\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(parse_tile)
        .collect()
}

fn rotate(grid: &Grid) -> Grid {
    let n = grid.len();
    (0..n)
        .map(|r| (0..n).map(|c| grid[n - 1 - c][r]).collect())
        .collect()
}

fn flip(grid: &Grid) -> Grid {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

/// All eight orientations: four rotations, each with and without a flip.
fn orientations(grid: &Grid) -> impl Iterator<Item = Grid> {
    iter::successors(Some(grid.clone()), |g| Some(rotate(g)))
        .take(4)
        .flat_map(|g| [flip(&g), g])
}

fn column(grid: &Grid, x: usize) -> Edge {
    grid.iter().map(|row| row[x]).collect()
}

fn edges(grid: &Grid) -> Edges {
    let last = grid.len() - 1;
    Edges {
        top: grid[0].clone(),
        bottom: grid[last].clone(),
        left: column(grid, 0),
        right: column(grid, last),
    }
}

/// Direction-independent form of an edge, so flipped tiles still compare equal.
fn normalise(edge: Edge) -> Edge {
    let reversed = edge.iter().rev().copied().collect();
    cmp::min(edge, reversed)
}

/// How many tiles carry each (normalised) edge; a count of one means the edge
/// has no partner and so lies on the outside of the image.
fn edge_counts(tiles: &[Tile]) -> HashMap<Edge, usize> {
    let mut counts = HashMap::new();
    for tile in tiles {
        for edge in edges(&tile.pixels).into_array() {
            *counts.entry(normalise(edge)).or_default() += 1;
        }
    }
    counts
}

/// Orients and places every tile, returning the grids in row-major order.
///
/// Relies on each border matching at most one other tile, so the first tile
/// that fits a slot is the right one. Slots on the top row / left column
/// demand an unmatched edge there, which also makes slot 0 pick a corner.
fn assemble(tiles: &[Tile], side: usize) -> Option<Vec<Grid>> {
    let counts = edge_counts(tiles);
    let is_outer = |edge: Edge| counts.get(&normalise(edge)) == Some(&1);
    let mut remaining: Vec<&Grid> = tiles.iter().map(|tile| &tile.pixels).collect();
    let mut placed: Vec<Grid> = Vec::with_capacity(tiles.len());
    for pos in 0..tiles.len() {
        let fits = |grid: &Grid| {
            let Edges { top, left, .. } = edges(grid);
            let left_ok = match pos % side {
                0 => is_outer(left),
                _ => left == edges(&placed[pos - 1]).right,
            };
            let top_ok = match pos.checked_sub(side) {
                None => is_outer(top),
                Some(above) => top == edges(&placed[above]).bottom,
            };
            left_ok && top_ok
        };
        let (index, grid) = remaining
            .iter()
            .enumerate()
            .find_map(|(i, tile)| orientations(tile).find(&fits).map(|grid| (i, grid)))?;
        remaining.swap_remove(index);
        placed.push(grid);
    }
    Some(placed)
}

/// Strips each placed tile's border and joins the interiors into one image.
fn stitch(placed: &[Grid], side: usize) -> Grid {
    placed
        .chunks(side)
        .flat_map(|tile_row| {
            let size = tile_row[0].len();
            (1..size - 1).map(move |y| {
                tile_row
                    .iter()
                    .flat_map(|tile| tile[y][1..size - 1].iter().copied())
                    .collect()
            })
        })
        .collect()
}

fn monster_offsets() -> Vec<(usize, usize)> {
    MONSTER
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

/// Every image cell covered by at least one sea monster.
fn monster_cells(image: &Grid, offsets: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    let is_set = |x: usize, y: usize| image.get(y).and_then(|row| row.get(x)) == Some(&true);
    (0..image.len())
        .flat_map(|y| (0..image.len()).map(move |x| (x, y)))
        .filter(|&(x, y)| offsets.iter().all(|&(dx, dy)| is_set(x + dx, y + dy)))
        .flat_map(|(x, y)| offsets.iter().map(move |&(dx, dy)| (x + dx, y + dy)))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse(input)?;
    let counts = edge_counts(&tiles);
    let corners: Vec<u64> = tiles
        .iter()
        .filter(|tile| {
            edges(&tile.pixels)
                .into_array()
                .into_iter()
                .filter(|edge| counts.get(&normalise(edge.clone())) == Some(&1))
                .count()
                == 2
        })
        .map(|tile| tile.id)
        .collect();
    (corners.len() == 4).then(|| corners.iter().product())
}

/// The assembled image in the orientation that shows sea monsters, together
/// with the cells those monsters cover.
fn habitat(input: &str) -> Option<(Grid, HashSet<(usize, usize)>)> {
    let tiles = parse(input)?;
    let side = (1..=tiles.len()).find(|n| n * n >= tiles.len())?;
    let image = stitch(&assemble(&tiles, side)?, side);
    let offsets = monster_offsets();
    orientations(&image).find_map(|oriented| {
        let cells = monster_cells(&oriented, &offsets);
        (!cells.is_empty()).then_some((oriented, cells))
    })
}

/// Draws the image as text: `O` for sea monster cells, `#` for rough water.
#[cfg(test)]
fn render(image: &Grid, monsters: &HashSet<(usize, usize)>) -> String {
    image
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &cell)| match (monsters.contains(&(x, y)), cell) {
                    (true, _) => 'O',
                    (false, true) => '#',
                    (false, false) => '.',
                })
                .chain(iter::once('\n'))
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (image, monsters) = habitat(input)?;
    let rough = image.iter().flatten().filter(|&&cell| cell).count() - monsters.len();
    Some(rough as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20899048083289));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(273));
    }

    #[test]
    fn test_render_marks_monsters() {
        let expected = [
            ".####...#####..#...###..",
            "#####..#..#.#.####..#.#.",
            ".#.#...#.###...#.##.O#..",
            "#.O.##.OO#.#.OO.##.OOO##",
            "..#O.#O#.O##O..O.#O##.##",
            "...#.#..##.##...#..#..##",
            "#.##.#..#.#..#..##.#.#..",
            ".###.##.....#...###.#...",
            "#.####.#.#....##.#..#.#.",
            "##...#..#....#..#...####",
            "..#.##...###..#.#####..#",
            "....#.##.#.#####....#...",
            "..##.##.###.....#.##..#.",
            "#...#...###..####....##.",
            ".#.##...#.##.#.#.###...#",
            "#.###.#..####...##..#...",
            "#.###...#.##...#.##O###.",
            ".O##.#OO.###OO##..OOO##.",
            "..O#.O..O..O.#O##O##.###",
            "#.#..##.########..#..##.",
            "#.#####..#.#...##..#....",
            "#....##..#.#########..##",
            "#...#.....#..##...###.##",
            "#..###....##.#...##.##.#",
        ];
        let input = advent_of_code::template::read_file("examples", DAY);
        let (image, monsters) = habitat(&input).unwrap();
        let rendered = render(&image, &monsters);
        assert_eq!(rendered.lines().collect::<Vec<_>>(), expected);
    }

    /// Writes the real input's image to `data/images/20.txt`.
    /// Run with `cargo test --bin 20 -- --ignored`.
    #[test]
    #[ignore = "writes a file; needs the real puzzle input"]
    fn write_image() {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let (image, monsters) = habitat(&input).unwrap();
        std::fs::create_dir_all("data/images").unwrap();
        std::fs::write("data/images/20.txt", render(&image, &monsters)).unwrap();
    }
}
