fn main() {
    //     let input = "MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX";
    let input = include_str!("../../../inputs/day-4.txt");

    let char_matrix = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = char_matrix.len();
    let cols = char_matrix[0].len();

    let mut res: usize = 0;

    let mut rows_matches: Vec<(usize, usize)> = Vec::new();
    for row in 0..(rows - 3) {
        let mut cols_matches: Vec<usize> = Vec::new();
        for col in 0..(cols - 3) {
            let m = [
                &char_matrix[row][col..(col + 4)],
                &char_matrix[row + 1][col..(col + 4)],
                &char_matrix[row + 2][col..(col + 4)],
                &char_matrix[row + 3][col..(col + 4)],
            ];
            // println!("---");
            // for a in m {
            //     println!("{:?}", a);
            // }

            let mut m_row_count = 0;
            for m_row in m {
                if m_row == ['X', 'M', 'A', 'S'] || m_row == ['S', 'A', 'M', 'X'] {
                    if !rows_matches.contains(&(row + m_row_count, col)) {
                        res += 1;
                        rows_matches.push((row + m_row_count, col));
                    }
                };
                m_row_count += 1;
            }

            let diag1 = [m[0][0], m[1][1], m[2][2], m[3][3]];
            if diag1 == ['X', 'M', 'A', 'S'] || diag1 == ['S', 'A', 'M', 'X'] {
                res += 1;
            };

            let diag2 = [m[3][0], m[2][1], m[1][2], m[0][3]];
            if diag2 == ['X', 'M', 'A', 'S'] || diag2 == ['S', 'A', 'M', 'X'] {
                res += 1;
            };

            for i in 0..4 {
                let m_col = [m[0][i], m[1][i], m[2][i], m[3][i]];
                if m_col == ['X', 'M', 'A', 'S'] || m_col == ['S', 'A', 'M', 'X'] {
                    if !cols_matches.contains(&(col + i)) {
                        res += 1;
                        cols_matches.push(col + i);
                    }
                };
            }
            // println!("count: {:?}, start [row: {row}, col: {col}]", res);
            // println!(
            //     "rows matches: {:?} cols matches: {:?}",
            //     rows_matches, cols_matches
            // );
        }
        // println!(
        //     "rows matches: {:?} cols matches: {:?}",
        //     rows_matches, cols_matches
        // );
    }

    println!("{res}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_1() {
        assert!(true)
    }
}
