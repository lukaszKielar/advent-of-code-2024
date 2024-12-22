pub mod part_1;
pub mod part_2;

const DO: &str = "do()";
const DONT: &str = "don't()";

#[derive(Debug)]
enum MulStatus {
    Disabled,
    Enabled,
}

#[derive(Debug, PartialEq)]
struct Mul(usize, usize);

impl Mul {
    fn result(&self) -> usize {
        self.0 * self.1
    }
}

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
