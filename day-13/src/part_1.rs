use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        // No solution
        return 0;
    }
    // Calculate price (3 * button a + button b)
    a * 3 + b
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|l| {
            let (x1, x2, y1, y2, z1, z2) = l
                .split(|c: char| !c.is_ascii_digit())
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Machine {
                button_a: (x1, y1),
                button_b: (x2, y2),
                prize: (z1, z2),
            }
        })
        .collect()
}

pub fn process(input: &str) -> usize {
    let machines = parse_input(input);
    let mut res = 0;

    for machine in machines {
        let (x1, y1) = machine.button_a;
        let (x2, y2) = machine.button_b;
        let (z1, z2) = machine.prize;
        res += solve(
            x1 as i64, x2 as i64, y1 as i64, y2 as i64, z1 as i64, z2 as i64,
        )
    }

    res as usize
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    }

    #[rstest]
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            vec![
                Machine {
                    button_a: (94, 34),
                    button_b: (22, 67),
                    prize: (8400, 5400)
                },
                Machine {
                    button_a: (26, 66),
                    button_b: (67, 21),
                    prize: (12748, 12176)
                },
                Machine {
                    button_a: (17, 86),
                    button_b: (84, 37),
                    prize: (7870, 6450)
                },
                Machine {
                    button_a: (69, 23),
                    button_b: (27, 71),
                    prize: (18641, 10279)
                }
            ]
        );
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 480)
    }
}
