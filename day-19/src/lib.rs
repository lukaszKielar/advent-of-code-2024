use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let patterns = split[0].split(", ").collect();
    let designs = split[1].lines().collect();

    (patterns, designs)
}

fn calculate_combinations<'d>(
    design: &'d str,
    patterns: &'_ Vec<&'_ str>,
    cache: &mut HashMap<&'d str, usize>,
) -> usize {
    if cache.get(design).is_none() {
        if design.is_empty() {
            return 1;
        }
        let mut res = 0;
        for pat in patterns {
            if let Some(rest) = design.strip_prefix(pat) {
                res += calculate_combinations(rest, patterns, cache);
            }
        }
        cache.insert(design, res);
    }
    // SAFETY: key exists by now
    *cache.get(design).unwrap()
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
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            (
                vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"],
                vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]
            )
        )
    }
}
