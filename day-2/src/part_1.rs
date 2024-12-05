use crate::{Report, MAX_DIFF, MIN_DIFF};

fn is_safe(levels: &Vec<usize>) -> bool {
    all_decreasing(levels) || all_increasing(levels)
}

fn all_increasing(levels: &Vec<usize>) -> bool {
    levels.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (w[0] < w[1]) && (diff >= MIN_DIFF && diff <= MAX_DIFF)
    })
}

fn all_decreasing(levels: &Vec<usize>) -> bool {
    levels.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (w[0] > w[1]) && (diff >= MIN_DIFF && diff <= MAX_DIFF)
    })
}

pub fn process(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| Report::from(line))
        .filter(|r| is_safe(&r.0))
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 2)
    }
}
