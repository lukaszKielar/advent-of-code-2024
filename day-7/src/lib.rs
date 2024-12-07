pub mod part_1;
pub mod part_2;

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let s = line.split(": ").collect::<Vec<_>>();
            let res = s[0].trim().parse::<usize>().unwrap();
            let values = s[1]
                .trim()
                .split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (res, values)
        })
        .collect::<Vec<_>>()
}

fn combine(
    values: &Vec<usize>,
    operators: &Vec<&str>,
    current: &mut Vec<String>,
    results: &mut Vec<String>,
) {
    if current.len() == values.len() - 1 {
        // Combine the values and operators to form a complete expression
        let mut expression = values[0].to_string();
        for (i, op) in current.iter().enumerate() {
            expression.push_str(&format!(" {} {}", op, values[i + 1]));
        }
        results.push(expression);
        return;
    }

    // Try each operator in the list
    for op in operators {
        current.push(op.to_string());
        combine(values, operators, current, results);
        current.pop(); // Backtrack to try other combinations
    }
}

fn generate_combinations(values: &Vec<usize>, operators: &Vec<&str>) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    combine(values, operators, &mut current, &mut results);
    results
}

fn calculate_expression(expression: &str) -> usize {
    let mut tokens = expression.split_whitespace();
    let mut result = tokens.next().unwrap().parse::<usize>().unwrap(); // First value

    while let Some(op) = tokens.next() {
        if let Some(value) = tokens.next() {
            let value = value.parse::<usize>().unwrap();
            match op {
                "+" => result += value,
                "*" => result *= value,
                _ => panic!("Unsupported operator: {}", op),
            }
        }
    }

    result
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
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ]
        )
    }

    #[rstest]
    fn test_generate_combinations() {
        // given
        let values = vec![9, 7, 18, 13];
        let operators = vec!["*", "+"];

        // when
        let output = generate_combinations(&values, &operators);

        // then
        assert_eq!(
            output,
            vec![
                "9 * 7 * 18 * 13",
                "9 * 7 * 18 + 13",
                "9 * 7 + 18 * 13",
                "9 * 7 + 18 + 13",
                "9 + 7 * 18 * 13",
                "9 + 7 * 18 + 13",
                "9 + 7 + 18 * 13",
                "9 + 7 + 18 + 13"
            ]
        )
    }

    #[rstest]
    #[case("81 * 40 * 27", 87480)]
    #[case("81 * 40 + 27", 3267)]
    #[case("81 + 40 * 27", 3267)]
    #[case("81 + 40 + 27", 148)]
    #[case("9 * 7 * 18 * 13", 14742)]
    #[case("9 * 7 * 18 + 13", 1147)]
    #[case("9 * 7 + 18 * 13", 1053)]
    #[case("9 * 7 + 18 + 13", 94)]
    #[case("9 + 7 * 18 * 13", 3744)]
    #[case("9 + 7 * 18 + 13", 301)]
    #[case("9 + 7 + 18 * 13", 442)]
    #[case("9 + 7 + 18 + 13", 47)]
    fn test_calculate_expression(#[case] expression: &str, #[case] expected: usize) {
        // when
        let output = calculate_expression(expression);

        // then
        assert_eq!(output, expected);
    }
}
