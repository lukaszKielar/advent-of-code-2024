pub mod part_1;
pub mod part_2;

const OBSTACLE: char = '#';

type Coords = (usize, usize);

type Matrix = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
struct Point {
    direction: Direction,
    coords: Coords,
}

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }
}

fn get_starting_point(matrix: &Matrix) -> Point {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let elem = matrix[i][j];
            if ['^', '>', 'v', '<'].contains(&elem) {
                let direction = match elem {
                    '^' => Direction::UP,
                    '>' => Direction::RIGHT,
                    'v' => Direction::DOWN,
                    '<' => Direction::LEFT,
                    _ => panic!("won't happen"),
                };
                return Point {
                    direction,
                    coords: (i, j),
                };
            }
        }
    }
    panic!("cannot find starting position")
}

fn parse_input(input: &str) -> Matrix {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn pretty_print(matrix: &Matrix) {
    println!("----------------------------");
    for i in 0..matrix.len() {
        println!("{:?}", matrix[i]);
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[fixture]
    fn matrix(input: &str) -> Matrix {
        parse_input(input)
    }

    #[rstest]
    fn test_get_starting_point(matrix: Matrix) {
        // when
        let output = get_starting_point(&matrix);

        // then
        assert_eq!(
            output,
            Point {
                direction: Direction::UP,
                coords: (6, 4)
            }
        )
    }

    #[rstest]
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // thena
        assert_eq!(
            output,
            [
                ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
            ]
        )
    }
}
