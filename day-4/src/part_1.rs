use crate::parse_input;

pub fn process(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    // Directions: (di, dj) for 8 possible directions
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    for i in 0..rows {
        for j in 0..cols {
            for &(di, dj) in &directions {
                if check_word(&grid, i, j, di, dj, "XMAS") {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word(grid: &[Vec<char>], i: i32, j: i32, di: i32, dj: i32, word: &str) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    for (k, &ch) in word_chars.iter().enumerate() {
        let ni = i + k as i32 * di;
        let nj = j + k as i32 * dj;
        if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
            return false;
        }
        if grid[ni as usize][nj as usize] != ch {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

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
        assert_eq!(output, 18)
    }
}
