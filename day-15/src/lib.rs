pub mod part_1;
pub mod part_2;

type Grid = Vec<Vec<char>>;
type Moves = Vec<Point>;

#[derive(Debug, PartialEq)]
struct Point {
    i: i32,
    j: i32,
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            i: value.0,
            j: value.1,
        }
    }
}

fn parse_input(input: &str) -> (Grid, Moves) {
    let input = input.trim().split("\n\n").collect::<Vec<_>>();

    let grid = input[0]
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("{grid:?}");

    let moves = input[1]
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '>' => (0, 1).into(),
                    '^' => (-1, 0).into(),
                    '<' => (0, -1).into(),
                    'v' => (1, 0).into(),
                    _ => panic!("it won't happen"),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    (grid, moves)
}

fn get_robot_position(grid: &Grid) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (i as i32, j as i32).into();
            }
        }
    }
    panic!("it won't happen")
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
    }

    #[rstest]
    fn test_parse_input() {
        // input
        let input = "####
#..#
#0.#
#00#
#@.#
####

>^<v
>^<v";

        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            (
                vec![
                    vec!['#', '#', '#', '#'],
                    vec!['#', '.', '.', '#'],
                    vec!['#', '0', '.', '#'],
                    vec!['#', '0', '0', '#'],
                    vec!['#', '@', '.', '#'],
                    vec!['#', '#', '#', '#'],
                ],
                vec![
                    (0, 1).into(),
                    (-1, 0).into(),
                    (0, -1).into(),
                    (1, 0).into(),
                    (0, 1).into(),
                    (-1, 0).into(),
                    (0, -1).into(),
                    (1, 0).into(),
                ]
            )
        )
    }
}
