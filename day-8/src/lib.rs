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
}
