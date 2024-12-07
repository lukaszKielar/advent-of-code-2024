use std::usize;

#[cfg(test)]
use crate::pretty_print;
use crate::{get_starting_point, move_guard_around, parse_input, Matrix};

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

    let solved_matrix = move_guard_around(matrix, point.coords, point.direction);
    #[cfg(test)]
    pretty_print(&solved_matrix);

    sum_all_x(&solved_matrix)
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

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 41)
    }
}
