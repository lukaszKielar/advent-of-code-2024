use crate::parse_input;

const MAS: [char; 3] = ['M', 'A', 'S'];
const ANTI_MAS: [char; 3] = ['S', 'A', 'M'];

fn get_3x3_x_mas_matrix_count(m: [&[char]; 3]) -> usize {
    let diag1 = [m[0][0], m[1][1], m[2][2]];
    let diag2 = [m[2][0], m[1][1], m[0][2]];

    if (diag1 == MAS || diag1 == ANTI_MAS) && (diag2 == MAS || diag2 == ANTI_MAS) {
        return 1;
    };

    0
}

pub fn process(input: &str) -> usize {
    let char_matrix = parse_input(input);

    let rows = char_matrix.len();
    let cols = char_matrix[0].len();

    let mut res: usize = 0;

    for row in 0..(rows - 2) {
        for col in 0..(cols - 2) {
            let m = [
                &char_matrix[row][col..(col + 3)],
                &char_matrix[row + 1][col..(col + 3)],
                &char_matrix[row + 2][col..(col + 3)],
            ];

            res += get_3x3_x_mas_matrix_count(m);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case([
        ['M', '.', 'S'],
        ['.', 'A', '.'],
        ['M', '.', 'S']
    ], 1)]
    #[case([
        ['S', '.', 'S'],
        ['.', 'A', '.'],
        ['M', '.', 'M']
    ], 1)]
    #[case([
        ['M', '.', 'M'],
        ['.', 'A', '.'],
        ['S', '.', 'S']
    ], 1)]
    #[case([
        ['S', '.', 'M'],
        ['.', 'A', '.'],
        ['S', '.', 'M']
    ], 1)]
    #[case([
        ['.', '.', 'M'],
        ['.', 'A', '.'],
        ['S', '.', 'M']
    ], 0)]
    #[case([
        ['.', '.', '.'],
        ['.', '.', '.'],
        ['.', '.', '.']
    ], 0)]
    fn test_get_3x3_x_mas_matrix_count(#[case] m: [[char; 3]; 3], #[case] expected: usize) {
        // when:
        let count = get_3x3_x_mas_matrix_count([&m[0][..], &m[1][..], &m[2][..]]);

        // then:
        assert_eq!(count, expected);
    }

    #[rstest]
    fn test_process() {
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
        let output = process(input);

        // then
        assert_eq!(output, 9)
    }
}
