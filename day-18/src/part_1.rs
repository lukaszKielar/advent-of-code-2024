use std::collections::VecDeque;

use crate::{parse_input, Coord};

fn bfs(start: Coord, end: Coord, grid: &Vec<Vec<char>>) -> Option<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut parent = vec![vec![None; grid[0].len()]; grid.len()];

    queue.push_back(start);
    visited[start.y as usize][start.x as usize] = true;

    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = Vec::new();
            let mut pos = Some(current);
            while let Some(p) = pos {
                path.push(p);
                pos = parent[p.y as usize][p.x as usize];
            }
            path.reverse();
            return Some(path);
        }

        for direction in [
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 0 },
            Coord { x: 0, y: -1 },
            Coord { x: -1, y: 0 },
        ] {
            let dx = current.x + direction.x;
            let dy = current.y + direction.y;

            if dy < 0 || dy >= grid.len() as i32 || dx < 0 || dx >= grid[0].len() as i32 {
                continue;
            }

            let next_pos: Coord = (dx, dy).into();

            if !visited[next_pos.y as usize][next_pos.x as usize]
                && grid[next_pos.y as usize][next_pos.x as usize] != '#'
            {
                visited[next_pos.y as usize][next_pos.x as usize] = true;
                parent[next_pos.y as usize][next_pos.x as usize] = Some(current);
                queue.push_back(next_pos);
            }
        }
    }

    None
}

pub fn process(input: &str, size: usize, bytes: usize) -> usize {
    let grid = parse_input(input, size, bytes);

    let start = (0, 0).into();
    let end = (size as i32, size as i32).into();

    bfs(start, end, &grid).unwrap().len() - 1
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input, 6, 12);

        // then
        assert_eq!(output, 22)
    }
}
