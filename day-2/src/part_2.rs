use crate::{Report, MAX_DIFF, MIN_DIFF};

fn is_safe(levels: &[usize], stop: bool) -> bool {
    if all_decreasing(levels) || all_increasing(levels) {
        true
    } else if stop {
        return false;
    } else {
        for i in 0..levels.len() {
            let mut filtered_levels = levels.to_vec();
            filtered_levels.remove(i);

            if is_safe(&filtered_levels, true) {
                return true;
            }
        }
        return false;
    }
}

fn all_increasing(levels: &[usize]) -> bool {
    levels.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (w[0] < w[1]) && (MIN_DIFF..=MAX_DIFF).contains(&diff)
    })
}

fn all_decreasing(levels: &[usize]) -> bool {
    levels.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (w[0] > w[1]) && (MIN_DIFF..=MAX_DIFF).contains(&diff)
    })
}

pub fn process(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(Report::from)
        .filter(|r| is_safe(&r.0, false))
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
        assert_eq!(output, 4)
    }
}
