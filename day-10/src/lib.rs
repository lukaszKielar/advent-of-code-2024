pub mod part_1;
pub mod part_2;

type Coords = (usize, usize);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[rstest]
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            [
                [8, 9, 0, 1, 0, 1, 2, 3],
                [7, 8, 1, 2, 1, 8, 7, 4],
                [8, 7, 4, 3, 0, 9, 6, 5],
                [9, 6, 5, 4, 9, 8, 7, 4],
                [4, 5, 6, 7, 8, 9, 0, 3],
                [3, 2, 0, 1, 9, 0, 1, 2],
                [0, 1, 3, 2, 9, 8, 0, 1],
                [1, 0, 4, 5, 6, 7, 3, 2]
            ]
        )
    }
}
