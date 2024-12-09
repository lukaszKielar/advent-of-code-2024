use std::usize;

use crate::{convert_to_blocks, move_blocks_around};

pub fn process(input: &str) -> usize {
    let blocks = convert_to_blocks(input.trim());
    // println!("blocks: {:?}", blocks);
    let blocks = move_blocks_around(&blocks);
    // println!("moved blocks: {:?}", blocks);

    let mut acc: usize = 0;

    for (i, c) in blocks.chars().enumerate() {
        acc += i * c.to_digit(10).unwrap_or(0) as usize;
    }

    acc
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
