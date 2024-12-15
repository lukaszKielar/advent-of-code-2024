use itertools::Itertools;

const CALIBRATION: i64 = 10000000000000;

#[derive(Debug, PartialEq)]
struct Coords {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Coords {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: Coords,
    button_b: Coords,
    prize: Coords,
}

impl Machine {
    // Cramer's rule
    fn solve(&self) -> i64 {
        let b = (self.prize.y * self.button_a.x - self.prize.x * self.button_a.y)
            / (self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y);
        let a = (self.prize.x - b * self.button_b.x) / self.button_a.x;
        if (
            self.button_a.x * a + self.button_b.x * b,
            self.button_a.y * a + self.button_b.y * b,
        ) != (self.prize.x, self.prize.y)
        {
            return 0;
        }
        a * 3 + b
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|l| {
            let (xa, ya, xb, yb, xp, yp) = l
                .split(|c: char| !c.is_ascii_digit())
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Machine {
                button_a: (xa, ya).into(),
                button_b: (xb, yb).into(),
                prize: (xp + CALIBRATION, yp + CALIBRATION).into(),
            }
        })
        .collect()
}

pub fn process(input: &str) -> usize {
    let machines = parse_input(input);
    let mut res = 0;

    for machine in machines {
        res += machine.solve()
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
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 875318608908)
    }
}
