use crate::parse_input;

pub fn process(input: &str) -> usize {
    let input = parse_input(input);

    let mut vec1 = input.iter().map(|elem| elem.0).collect::<Vec<_>>();
    vec1.sort();

    let mut vec2 = input.iter().map(|elem| elem.1).collect::<Vec<_>>();
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
