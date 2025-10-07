use std::collections::HashSet;

use crate::{parse_input, X, Y};

pub fn process(input: &str) -> usize {
    let robots = parse_input(input);

    for seconds in 1..=10000 {
        let mut positions = HashSet::new();
        let mut temp_robots = robots.clone();

        for robot in temp_robots.iter_mut() {
            robot.move_sec(seconds, &(X, Y));
            positions.insert(robot.p);
        }

        // If all robots are in unique positions, this might be our pattern
        if positions.len() == robots.len() {
            return seconds as usize;
        }
    }

    0 // Should not reach here if solution exists
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
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        // Part 2 doesn't provide expected output in the problem description
        // We just verify it returns a non-zero value
        assert!(output > 0);
    }
}
