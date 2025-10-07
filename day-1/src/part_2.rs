use std::collections::HashMap;

use crate::parse_input;

pub fn process(input: &str) -> usize {
    let (vec1, vec2) = parse_input(input);

    let vec2_hashmap =
        vec2
            .iter()
            .fold(HashMap::<usize, usize>::new(), |mut m, &x| {
                *m.entry(x).or_insert(0) += 1;
                m
            });

    vec1
        .iter()
        .map(|&elem| {
            let occurences = vec2_hashmap.get(&elem).unwrap_or(&0);
            elem * occurences
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
