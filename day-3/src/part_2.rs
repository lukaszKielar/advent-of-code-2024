use regex::Regex;

use crate::{Mul, MulStatus, DO, DONT};

pub fn process(input: &str) -> usize {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((?<v1>[0-9]{1,3})\,(?<v2>[0-9]{1,3})\))").unwrap();

    let matches = re.find_iter(input.trim()).collect::<Vec<_>>();

    let mut res = 0;

    let mut mul_status = MulStatus::Enabled;
    for m in matches {
        if m.as_str() == DO {
            mul_status = MulStatus::Enabled;
        } else if m.as_str() == DONT {
            mul_status = MulStatus::Disabled;
        } else {
            match mul_status {
                MulStatus::Disabled => continue,
                MulStatus::Enabled => {
                    let mul = Mul::from(m.as_str());
                    res += mul.result();
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 48)
    }
}
