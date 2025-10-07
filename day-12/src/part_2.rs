use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EdgeKind {
    Top,
    Bottom,
    Left,
    Right,
}

type Grid = Vec<Vec<char>>;
type Coords = (usize, usize);
type Regions = HashMap<usize, Region>;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug)]
struct Region {
    points: HashSet<Coords>,
}

impl Region {
    fn new() -> Self {
        Self {
            points: HashSet::new(),
        }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn sides(&self) -> usize {
        let mut edge_map: HashMap<(EdgeKind, usize), Vec<usize>> = HashMap::new();

        for &(i, j) in &self.points {
            // top edge
            match i.checked_sub(1) {
                Some(up) => {
                    if !self.points.contains(&(up, j)) {
                        edge_map
                            .entry((EdgeKind::Top, i))
                            .or_default()
                            .push(j);
                    }
                }
                None => {
                    edge_map
                        .entry((EdgeKind::Top, i))
                        .or_default()
                        .push(j);
                }
            }

            // bottom edge
            match i.checked_add(1) {
                Some(down) => {
                    if !self.points.contains(&(down, j)) {
                        edge_map
                            .entry((EdgeKind::Bottom, down))
                            .or_default()
                            .push(j);
                    }
                }
                None => {
                    edge_map
                        .entry((EdgeKind::Bottom, i.saturating_add(1)))
                        .or_default()
                        .push(j);
                }
            }

            // left edge
            match j.checked_sub(1) {
                Some(left) => {
                    if !self.points.contains(&(i, left)) {
                        edge_map
                            .entry((EdgeKind::Left, j))
                            .or_default()
                            .push(i);
                    }
                }
                None => {
                    edge_map
                        .entry((EdgeKind::Left, j))
                        .or_default()
                        .push(i);
                }
            }

            // right edge
            match j.checked_add(1) {
                Some(right) => {
                    if !self.points.contains(&(i, right)) {
                        edge_map
                            .entry((EdgeKind::Right, right))
                            .or_default()
                            .push(i);
                    }
                }
                None => {
                    edge_map
                        .entry((EdgeKind::Right, j.saturating_add(1)))
                        .or_default()
                        .push(i);
                }
            }
        }

        edge_map
            .values()
            .map(|positions| count_runs(positions.clone()))
            .sum()
    }

    fn price(&self) -> usize {
        self.area() * self.sides()
    }
}

fn find_region(
    i: usize,
    j: usize,
    grid: &Grid,
    visited: &mut HashSet<Coords>,
    region: &mut Region,
) {
    if visited.contains(&(i, j)) {
        return;
    }

    visited.insert((i, j));
    region.points.insert((i, j));

    for (di, dj) in DIRECTIONS {
        let next_i = i as i32 + di;
        let next_j = j as i32 + dj;

        if next_i < 0
            || next_i as usize >= grid.len()
            || next_j < 0
            || next_j as usize >= grid[0].len()
        {
            continue;
        }

        // we know it's positive number by now
        let next_i = next_i as usize;
        let next_j = next_j as usize;

        if grid[i][j] != grid[next_i][next_j] {
            continue;
        }

        find_region(next_i, next_j, grid, visited, region);
    }
}

pub fn process(input: &str) -> usize {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut res = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut region_id: usize = 0;
    let mut regions: Regions = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            let mut region = Region::new();

            find_region(i, j, &grid, &mut visited, &mut region);

            regions.entry(region_id).or_insert(region);
            region_id += 1;
        }
    }

    for (_, r) in regions {
        res += r.price()
    }

    res
}

fn count_runs(mut positions: Vec<usize>) -> usize {
    if positions.is_empty() {
        return 0;
    }

    positions.sort_unstable();
    positions.dedup();

    let mut sides = 1;
    for k in 1..positions.len() {
        if positions[k] != positions[k - 1] + 1 {
            sides += 1;
        }
    }

    sides
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 1206)
    }
}
