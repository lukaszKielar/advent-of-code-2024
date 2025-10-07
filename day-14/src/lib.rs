use regex::Regex;

pub mod part_1;
pub mod part_2;

pub const X: usize = 101;
pub const Y: usize = 103;

#[derive(Debug, PartialEq, Clone)]
struct Robot {
    p: (usize, usize),
    v: (isize, isize),
}

impl Robot {
    fn move_sec(&mut self, sec: u32, grid: &(usize, usize)) {
        let mut new_px = (self.p.0 as isize + self.v.0 * sec as isize) % grid.0 as isize;
        let mut new_py = (self.p.1 as isize + self.v.1 * sec as isize) % grid.1 as isize;

        if new_px < 0 {
            new_px += grid.0 as isize;
        }

        if new_py < 0 {
            new_py += grid.1 as isize;
        }

        self.p.0 = new_px as usize;
        self.p.1 = new_py as usize;
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let re =
            Regex::new(r"^p=(?<px>\d+),(?<py>\d+) v=(?<vx>-{0,1}\d+),(?<vy>-{0,1}\d+)$").unwrap();
        let caps = re.captures(value).unwrap();

        Self {
            p: (caps["px"].parse().unwrap(), caps["py"].parse().unwrap()),
            v: (caps["vx"].parse().unwrap(), caps["vy"].parse().unwrap()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.trim().lines().map(Robot::from).collect()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[rstest]
    fn test_robot_from_str() {
        // given
        let input = "p=52,66 v=37,34";

        // when
        let output = Robot::from(input);

        // then
        assert_eq!(
            output,
            Robot {
                p: (52, 66),
                v: (37, 34)
            }
        )
    }

    #[rstest]
    #[case(1, (4,1))]
    #[case(2, (6,5))]
    #[case(3, (8,2))]
    #[case(4, (10,6))]
    #[case(5, (1,3))]
    fn test_robot_move_sec(#[case] sec: u32, #[case] expected: (usize, usize)) {
        // given
        let mut robot = Robot::from("p=2,4 v=2,-3");

        // when
        robot.move_sec(sec, &(11, 7));

        // then
        assert_eq!(robot.p, expected);
    }

    #[rstest]
    fn test_parse_input(input: &str) {
        // when
        let output = parse_input(input);

        // then
        assert_eq!(
            output,
            vec![
                Robot {
                    p: (0, 4),
                    v: (3, -3)
                },
                Robot {
                    p: (6, 3),
                    v: (-1, -3)
                },
                Robot {
                    p: (10, 3),
                    v: (-1, 2)
                },
                Robot {
                    p: (2, 0),
                    v: (2, -1)
                },
                Robot {
                    p: (0, 0),
                    v: (1, 3)
                },
                Robot {
                    p: (3, 0),
                    v: (-2, -2)
                },
                Robot {
                    p: (7, 6),
                    v: (-1, -3)
                },
                Robot {
                    p: (3, 0),
                    v: (-1, -2)
                },
                Robot {
                    p: (9, 3),
                    v: (2, 3)
                },
                Robot {
                    p: (7, 3),
                    v: (-1, 2)
                },
                Robot {
                    p: (2, 4),
                    v: (2, -3)
                },
                Robot {
                    p: (9, 5),
                    v: (-3, -3)
                },
            ]
        )
    }
}
