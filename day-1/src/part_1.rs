use crate::parse_input;

pub fn process(input: &str) -> usize {
    let (mut vec1, mut vec2) = parse_input(input);
    vec1.sort();
    vec2.sort();

    std::iter::zip(vec1, vec2)
        .map(|elems| elems.0.abs_diff(elems.1))
        .sum()
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 11)
    }
}
