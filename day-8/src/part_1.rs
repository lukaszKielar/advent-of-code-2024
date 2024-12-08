use crate::{find_antennas, find_antinodes, parse_input};

pub fn process(input: &str) -> usize {
    let matrix = parse_input(input);
    let antennas = find_antennas(&matrix);
    let antinodes = find_antinodes(&matrix, &antennas);

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#."
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 14)
    }
}
