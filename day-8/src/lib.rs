use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub mod part_1;
pub mod part_2;

type Coords = (i32, i32);
type Matrix = Vec<Vec<char>>;

fn parse_input(input: &str) -> Matrix {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_antennas(matrix: &Matrix) -> HashMap<char, HashSet<Coords>> {
    let mut matches = HashMap::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            match matrix[i][j] {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    matches
                        .entry(matrix[i][j])
                        .and_modify(|set: &mut HashSet<Coords>| {
                            set.insert((i as i32, j as i32));
                        })
                        .or_insert(HashSet::from([(i as i32, j as i32)]));
                }
                _ => continue,
            }
        }
    }

    matches
}

fn find_antinodes(matrix: &Matrix, antennas: &HashMap<char, HashSet<Coords>>) -> HashSet<Coords> {
    let mut antinodes = HashSet::new();

    for a in antennas.values() {
        for combo in a.into_iter().combinations(2).collect::<Vec<_>>() {
            let a1 = combo[0];
            let a2 = combo[1];

            let diff: Coords = (a1.0 - a2.0, a1.1 - a2.1);

            let n1: Coords = (a1.0 - 2 * diff.0, a1.1 - 2 * diff.1);
            if n1.0 >= 0 && n1.0 < matrix.len() as i32 && n1.1 >= 0 && n1.1 < matrix[0].len() as i32
            {
                antinodes.insert(n1);
            }

            let n2: Coords = (a1.0 + diff.0, a1.1 + diff.1);
            if n2.0 >= 0 && n2.0 < matrix.len() as i32 && n2.1 >= 0 && n2.1 < matrix[0].len() as i32
            {
                antinodes.insert(n2);
            }

            // println!("a1: {:?}, a2: {:?}", a1, a2);
            // println!("n1: {:?}, n2: {:?}", n1, n2);
        }
    }

    antinodes
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

    #[fixture]
    fn matrix(input: &str) -> Matrix {
        parse_input(input)
    }

    #[rstest]
    fn test_find_antennas(matrix: Matrix) {
        // when
        let output = find_antennas(&matrix);

        // then
        assert_eq!(
            output,
            HashMap::from([
                ('0', HashSet::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
                ('A', HashSet::from([(5, 6), (8, 8), (9, 9)])),
            ])
        )
    }

    #[rstest]
    fn test_find_antinodes(matrix: Matrix) {
        // given
        let antennas = find_antennas(&matrix);
        println!("antennas: {:?}", antennas);

        // when
        let output = find_antinodes(&matrix, &antennas);

        // then
        assert_eq!(
            output.into_iter().sorted().collect::<Vec<(i32, i32)>>(),
            vec![
                (0, 6),
                (0, 11),
                (1, 3),
                (2, 4),
                (2, 10),
                (3, 2),
                (4, 9),
                (5, 1),
                (5, 6),
                (6, 3),
                (7, 0),
                (7, 7),
                (10, 10),
                (11, 10)
            ]
        )
    }
}
