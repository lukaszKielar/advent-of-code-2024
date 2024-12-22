pub(crate) fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<_>>();
            (
                line[0].parse::<usize>().unwrap(),
                line[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_parse_input() {
        // given
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        // when
        let output = parse_input(input);

        // then
        assert_eq!(output, vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)]);
    }
}
