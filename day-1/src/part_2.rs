use std::collections::HashMap;

use crate::parse_input;

pub fn process(input: &str) -> usize {
    let input = parse_input(input);

    let vec2_hashmap =
        input
            .iter()
            .map(|elem| elem.1)
            .fold(HashMap::<usize, usize>::new(), |mut m, x| {
                *m.entry(x).or_insert(0) += 1;
                m
            });

    input
        .iter()
        .map(|elem| {
            let occurences = vec2_hashmap.get(&elem.0).unwrap_or(&0);
            elem.0 * occurences
        })
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
        assert_eq!(output, 31)
    }
}
