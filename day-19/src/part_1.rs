use std::collections::HashMap;

use crate::parse_input;

fn calculate_combinations<'d>(
    design: &'d str,
    patterns: &'_ Vec<&'_ str>,
    cache: &mut HashMap<&'d str, usize>,
) -> usize {
    if cache.get(design).is_none() {
        if design.len() == 0 {
            return 1;
        }
        let mut res = 0;
        for pat in patterns {
            if design.starts_with(pat) {
                res += calculate_combinations(&design[pat.len()..], patterns, cache);
            }
        }
        cache.insert(design, res);
    }
    // SAFETY: key exists by now
    *cache.get(design).unwrap()
}

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
