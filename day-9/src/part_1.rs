use std::usize;

use crate::{convert_to_blocks, move_blocks_around, GAP};

pub fn process(input: &str) -> usize {
    let blocks = convert_to_blocks(input.trim());
    let blocks = move_blocks_around(blocks);

    blocks
        .iter()
        .filter(|&&elem| elem != GAP)
        .enumerate()
        .fold(0, |acc, (i, &id)| acc + i * id as usize)
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
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 1928);
    }
}
