#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::process;

    #[fixture]
    fn input() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[rstest]
    fn test_process(input: &str) {
        // given
        let operators = vec!["*", "+"];

        // when
        let output = process(input, &operators);

        // then
        assert_eq!(output, 3749)
    }
}
