pub(crate) fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip()
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
        let (a, b) = parse_input(input);

        // then
        assert_eq!(a, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(b, vec![4, 3, 5, 3, 9, 3]);
    }
}
