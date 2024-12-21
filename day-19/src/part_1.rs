use std::collections::HashMap;

use crate::{calculate_combinations, parse_input};

pub fn process(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut res = 0;
    let mut cache = HashMap::new();

    for design in designs {
        if calculate_combinations(design, &patterns, &mut cache) > 0 {
            res += 1
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 6)
    }
}
