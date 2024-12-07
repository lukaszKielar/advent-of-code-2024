use crate::{calculate_guard_movement, get_starting_coords, parse_input};

pub fn process(input: &str) -> usize {
    let matrix = parse_input(input);
    let start_coords = get_starting_coords(&matrix);
    let guard_movement = calculate_guard_movement(&matrix, start_coords);

    guard_movement.len()
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
