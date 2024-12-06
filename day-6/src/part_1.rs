use std::usize;

use crate::{get_starting_point, parse_input, Coords, Direction, Matrix, OBSTACLE};

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
        return move_guard_around(matrix, coords, direction.next());
    } else {
        return move_guard_around(matrix, next_coords, direction);
    }
}

fn sum_all_x(matrix: &Matrix) -> usize {
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                count += 1;
            }
        }
    }
    count
}

pub fn process(input: &str) -> usize {
    let matrix = parse_input(input);

    let point = get_starting_point(&matrix);

    let matrix = move_guard_around(matrix, point.coords, point.direction);

    sum_all_x(&matrix)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 41)
    }
}
