use std::collections::HashSet;

pub mod part_1;
pub mod part_2;

// assuming that first direction moves guard upwards
// [UP, RIGHT, DOWN, LEFT]
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const START: char = '^';
const OBSTACLE: char = '#';

type Coords = (i32, i32);
type Matrix = Vec<Vec<char>>;

fn get_starting_coords(matrix: &Matrix) -> Coords {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == START {
                return (i as i32, j as i32);
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

fn calculate_guard_movement(matrix: &Matrix, (i, j): Coords) -> HashSet<Coords> {
    let mut directions = DIRECTIONS.iter().cycle();
    let (mut delta_i, mut delta_j): &(i32, i32) = directions.next().unwrap();
    let mut visited_cells: HashSet<(i32, i32)> = HashSet::from([(i, j)]);
    let (mut i, mut j): (i32, i32) = (i, j);

    loop {
        let (next_i, next_j) = (i + delta_i, j + delta_j);

        // return if out of bounds
        let cell = match matrix
            .get(next_i as usize)
            .and_then(|row| row.get(next_j as usize))
        {
            Some(&c) => c,
            None => return visited_cells,
        };

        match cell {
            // switch direction, but keep current position if obstacle
            OBSTACLE => (delta_i, delta_j) = *directions.next().unwrap(),
            // still within bounds and got cell that is legit
            '.' | START => {
                visited_cells.insert((next_i, next_j));
                (i, j) = (next_i, next_j);
            }
            c => panic!("unknown cell: {c}"),
        }
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
    fn starting_coords(matrix: Matrix) -> Coords {
        get_starting_coords(&matrix)
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

    #[rstest]
    fn test_get_starting_coords(matrix: Matrix) {
        // when
        let output = get_starting_coords(&matrix);

        // then
        assert_eq!(output, (6, 4))
    }

    #[rstest]
    fn test_calculate_guard_movement(matrix: Matrix, starting_coords: Coords) {
        // when
        let output = calculate_guard_movement(&matrix, starting_coords);

        // then
        assert_eq!(
            output,
            HashSet::from([
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (2, 4),
                (2, 8),
                (3, 4),
                (3, 8),
                (4, 2),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
                (4, 8),
                (5, 2),
                (5, 4),
                (5, 6),
                (5, 8),
                (6, 2),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
                (6, 7),
                (6, 8),
                (7, 1),
                (7, 2),
                (7, 3),
                (7, 4),
                (7, 5),
                (7, 6),
                (7, 7),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (8, 6),
                (8, 7),
                (9, 7),
            ])
        )
    }
}
