use std::collections::{HashMap, HashSet};

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
        todo!()
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
