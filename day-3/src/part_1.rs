use regex::Regex;

use crate::Mul;

pub fn process(input: &str) -> usize {
    let re = Regex::new(r"mul\((?<v1>[0-9]{1,3})\,(?<v2>[0-9]{1,3})\)").unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|caps| {
                    let v1 = caps.name("v1").unwrap().as_str().parse::<usize>().unwrap();
                    let v2 = caps.name("v2").unwrap().as_str().parse::<usize>().unwrap();
                    Mul(v1, v2)
                })
                .map(|elem| elem.result())
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 161)
    }
}
