pub mod part_1;
pub mod part_2;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let patterns = split[0].split(", ").collect();
    let designs = split[1].lines().collect();

    (patterns, designs)
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
