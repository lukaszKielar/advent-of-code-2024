use std::collections::HashSet;

use crate::{
    calculate_guard_movement, get_starting_coords, parse_input, Coords, Matrix, DIRECTIONS,
    OBSTACLE, START,
};

fn calculate_obstacles(matrix: &Matrix, (i, j): Coords, guard_movement: &HashSet<Coords>) -> usize {
    let mut infinite_loop_counter: usize = 0;

    // obstacle has to be placed only on the guard's movement path
    for obstacle in guard_movement {
        let mut stop_conditions: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut directions = DIRECTIONS.iter().cycle();
        let (mut delta_i, mut delta_j): &(i32, i32) = directions.next().unwrap();
        let (mut i, mut j): (i32, i32) = (i, j);

        loop {
            let (next_i, next_j) = (i + delta_i, j + delta_j);

            // break if out of bounds, otherwise get cell
            let mut cell = match matrix
                .get(next_i as usize)
                .and_then(|row| row.get(next_j as usize))
            {
                Some(&c) => c,
                None => break,
            };

            if (next_i, next_j) == *obstacle {
                cell = OBSTACLE;
            }

            match cell {
                OBSTACLE => {
                    if stop_conditions.contains(&((next_i, next_j), (delta_i, delta_j))) {
                        infinite_loop_counter += 1;
                        break;
                    }

                    stop_conditions.push(((next_i, next_j), (delta_i, delta_j)));
                    (delta_i, delta_j) = *directions.next().unwrap();
                }
                '.' | START => (i, j) = (next_i, next_j),
                c => panic!("unknown cell: {c}"),
            }
        }
    }

    infinite_loop_counter
}

pub fn process(input: &str) -> usize {
    let matrix = parse_input(input);
    let start_coords = get_starting_coords(&matrix);
    let guard_movement = calculate_guard_movement(&matrix, start_coords);

    calculate_obstacles(&matrix, start_coords, &guard_movement)
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
        assert_eq!(output, 6)
    }
}
