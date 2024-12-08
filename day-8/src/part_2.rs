use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{find_antennas, parse_input, Coords, Matrix};

fn find_antinodes(matrix: &Matrix, antennas: &HashMap<char, HashSet<Coords>>) -> HashSet<Coords> {
    let mut antinodes = HashSet::new();

    for a in antennas.values() {
        for combo in a.into_iter().combinations(2).collect::<Vec<_>>() {
            let a1 = combo[0];
            let a2 = combo[1];

            antinodes.insert(a1.clone());
            antinodes.insert(a2.clone());

            let diff: Coords = (a1.0 - a2.0, a1.1 - a2.1);

            let mut iter = 0;
            loop {
                let n1: Coords = (a1.0 - ((2 + iter) * diff.0), a1.1 - ((2 + iter) * diff.1));
                if n1.0 >= 0
                    && n1.0 < matrix.len() as i32
                    && n1.1 >= 0
                    && n1.1 < matrix[0].len() as i32
                {
                    antinodes.insert(n1);
                } else {
                    break;
                }

                iter += 1;
            }

            iter = 1;
            loop {
                let n2: Coords = (a1.0 + (iter * diff.0), a1.1 + (iter * diff.1));
                if n2.0 >= 0
                    && n2.0 < matrix.len() as i32
                    && n2.1 >= 0
                    && n2.1 < matrix[0].len() as i32
                {
                    antinodes.insert(n2);
                } else {
                    break;
                }
                iter += 1;
            }
        }
    }

    antinodes
}

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
        "##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##"
    }

    #[fixture]
    fn matrix(input: &str) -> Matrix {
        parse_input(input)
    }

    #[rstest]
    fn test_find_antinodes(matrix: Matrix) {
        // given
        let antennas = find_antennas(&matrix);

        // when
        let output = find_antinodes(&matrix, &antennas);

        // then
        assert_eq!(
            output.into_iter().sorted().collect::<Vec<(i32, i32)>>(),
            vec![
                (0, 0),
                (0, 1),
                (0, 6),
                (0, 11),
                (1, 1),
                (1, 3),
                (1, 8),
                (2, 2),
                (2, 4),
                (2, 5),
                (2, 10),
                (3, 2),
                (3, 3),
                (3, 7),
                (4, 4),
                (4, 9),
                (5, 1),
                (5, 5),
                (5, 6),
                (5, 11),
                (6, 3),
                (6, 6),
                (7, 0),
                (7, 5),
                (7, 7),
                (8, 2),
                (8, 8),
                (9, 4),
                (9, 9),
                (10, 1),
                (10, 10),
                (11, 3),
                (11, 10),
                (11, 11)
            ]
        )
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 34)
    }
}
