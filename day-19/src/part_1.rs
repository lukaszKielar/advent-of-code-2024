use crate::parse_input;

pub fn process(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut res = 0;

    for design in designs {
        println!("{design:?}");
        let mut design = design.to_owned();
        let mut chars = 0;

        while !design.is_empty() {
            println!("\t{design:?}, {chars}");
            let ref pat = &design[..design.len() - chars];
            if patterns.contains(pat) {
                design = design[design.len() - chars..].to_owned();
                chars = 0;
            } else {
                if chars >= design.len() {
                    break;
                }
                chars += 1;
            }
        }

        if design.is_empty() {
            println!("\tdone");
            res += 1;
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
