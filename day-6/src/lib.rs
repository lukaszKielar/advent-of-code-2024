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

impl Point {
    fn next_point_coords(&self) -> Coords {
        match self.direction {
            Direction::UP => (self.coords.0 - 1, self.coords.1),
            Direction::DOWN => (self.coords.0 + 1, self.coords.1),
            Direction::LEFT => (self.coords.0, self.coords.1 - 1),
            Direction::RIGHT => (self.coords.0, self.coords.1 + 1),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_right(&self) -> Direction {
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
                    _ => panic!("invalid direction"),
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

#[cfg(test)]
fn pretty_print(matrix: &Matrix) {
    println!("{}", (0..10).map(|_| '-').collect::<String>());
    for i in 0..matrix.len() {
        println!("{}", matrix[i].iter().collect::<String>());
    }
    println!("")
}

fn move_guard_around(matrix: Matrix, coords: Coords, direction: Direction) -> Matrix {
    // pretty_print(&matrix);
    let mut matrix = matrix;
    matrix[coords.0][coords.1] = 'X';

    let next_coords = match direction {
        Direction::UP => (coords.0 - 1, coords.1),
        Direction::DOWN => (coords.0 + 1, coords.1),
        Direction::LEFT => (coords.0, coords.1 - 1),
        Direction::RIGHT => (coords.0, coords.1 + 1),
    };

    if next_coords.0 == usize::MAX
        || next_coords.0 >= matrix.len()
        || next_coords.1 == usize::MAX
        || next_coords.1 >= matrix[0].len()
    {
        return matrix;
    };

    if matrix[next_coords.0][next_coords.1] == OBSTACLE {
        return move_guard_around(matrix, coords, direction.turn_right());
    } else {
        return move_guard_around(matrix, next_coords, direction);
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

    #[fixture]
    fn starting_point(matrix: Matrix) -> Point {
        get_starting_point(&matrix)
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
    fn test_move_guard_around(matrix: Matrix, starting_point: Point) {
        // when
        let output = move_guard_around(matrix, starting_point.coords, starting_point.direction);

        // then
        assert_eq!(
            output,
            [
                ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', 'X', 'X', 'X', 'X', 'X', '#'],
                ['.', '.', '.', '.', 'X', '.', '.', '.', 'X', '.'],
                ['.', '.', '#', '.', 'X', '.', '.', '.', 'X', '.'],
                ['.', '.', 'X', 'X', 'X', 'X', 'X', '#', 'X', '.'],
                ['.', '.', 'X', '.', 'X', '.', 'X', '.', 'X', '.'],
                ['.', '#', 'X', 'X', 'X', 'X', 'X', 'X', 'X', '.'],
                ['.', 'X', 'X', 'X', 'X', 'X', 'X', 'X', '#', '.'],
                ['#', 'X', 'X', 'X', 'X', 'X', 'X', 'X', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '#', 'X', '.', '.']
            ]
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
