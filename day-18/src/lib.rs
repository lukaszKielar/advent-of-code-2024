pub mod part_1;
pub mod part_2;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn parse_input(input: &str, size: usize, bytes: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; size + 1]; size + 1];

    let points: Vec<Coord> = input
        .trim()
        .lines()
        .map(|line| {
            let values = line.split(',').collect::<Vec<_>>();

            (
                values[0].parse::<i32>().unwrap(),
                values[1].parse::<i32>().unwrap(),
            )
                .into()
        })
        .collect::<Vec<_>>();

    for p in &points[0..bytes] {
        grid[p.y as usize][p.x as usize] = '#'
    }

    grid
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
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input, 6, 12);

        // then
        assert_eq!(
            output,
            [
                ['.', '.', '.', '#', '.', '.', '.'],
                ['.', '.', '#', '.', '.', '#', '.'],
                ['.', '.', '.', '.', '#', '.', '.'],
                ['.', '.', '.', '#', '.', '.', '#'],
                ['.', '.', '#', '.', '.', '#', '.'],
                ['.', '#', '.', '.', '#', '.', '.'],
                ['#', '.', '#', '.', '.', '.', '.']
            ]
        )
    }
}
