use std::usize;

use crate::{checksum, convert_to_blocks, Blocks, GAP};

fn move_blocks_around(blocks: Blocks) -> Blocks {
    let mut blocks = blocks;

    for i in 0..blocks.len() {
        let mut j = blocks.len() - 1;
        match blocks[i] {
            GAP => {
                while j > i {
                    match blocks[j] {
                        GAP => {
                            j -= 1;
                        }
                        _ => {
                            blocks[i] = blocks[j];
                            blocks[j] = GAP;
                            break;
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    blocks
}

pub fn process(input: &str) -> usize {
    let blocks = convert_to_blocks(input.trim());
    let blocks = move_blocks_around(blocks);

    checksum(blocks)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "2333133121414131402"
    }

    #[rstest]
    fn test_move_blocks_around(input: &str) {
        // given
        let blocks = convert_to_blocks(input);

        // when
        let output = move_blocks_around(blocks);

        // then
        assert_eq!(
            output,
            [
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
            ]
        )
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 1928);
    }
}
