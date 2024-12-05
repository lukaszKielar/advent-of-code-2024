pub mod part_1;
pub mod part_2;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_parse_input() {
        // given
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            [
                ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
            ]
        )
    }
}
