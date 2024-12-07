use crate::{calculate_expression, generate_combinations, parse_input};

pub fn process(input: &str) -> usize {
    let operators = vec!["*", "+"];
    let input = parse_input(input);
    let mut res = 0;

    for i in input {
        let combinations = generate_combinations(&i.1, &operators);
        for c in combinations {
            if calculate_expression(&c) == i.0 {
                res += i.0;
                break;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

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
        // when
        let output = process(input);

        // then
        assert_eq!(output, 3749)
    }
}
