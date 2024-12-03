use regex::Regex;

const DO: &'static str = "do()";
const DONT: &'static str = "don't()";

#[derive(Debug, PartialEq)]
struct Mul(usize, usize);

impl From<&str> for Mul {
    fn from(value: &str) -> Self {
        let v = value
            .strip_prefix("mul(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .collect::<Vec<_>>();

        Self(
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<usize>().unwrap(),
        )
    }
}

#[derive(Debug)]
enum MulStatus {
    Disabled,
    Enabled,
}

impl Mul {
    fn result(&self) -> usize {
        self.0 * self.1
    }
}

fn main() {
    //     let input = "
    // xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // ";
    let input = include_str!("../../../inputs/day-3.txt");

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((?<v1>[0-9]{1,3})\,(?<v2>[0-9]{1,3})\))").unwrap();

    let matches = input
        .trim()
        .lines()
        .map(|line| re.find_iter(line).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

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

    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_mul_from_str() {
        // given
        let v = "mul(1,234)";

        // when
        let m = Mul::from(v);

        // then
        assert_eq!(m, Mul(1, 234));
    }
}
