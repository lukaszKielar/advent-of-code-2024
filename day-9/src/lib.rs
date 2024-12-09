pub mod part_1;
pub mod part_2;

const GAP: i32 = -1;

type Blocks = Vec<i32>;

fn convert_to_blocks(input: &str) -> Blocks {
    let mut output = Vec::new();
    let mut id = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap();
        for _ in 0..num {
            if i % 2 == 0 {
                output.push(id);
            } else {
                output.push(GAP);
            }
        }

        if i % 2 == 0 {
            id += 1;
        }
    }

    output
}

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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "2333133121414131402"
    }

    #[rstest]
    fn test_convert_to_blocks(input: &str) {
        // when
        let output = convert_to_blocks(input);

        // then
        assert_eq!(
            output,
            [
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        )
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
}
