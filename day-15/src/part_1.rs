use crate::{get_robot_position, parse_input};

pub fn process(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input);
    let mut robot = get_robot_position(&grid);

    for m in moves {
        println!(
            "before move: {:?}, robot [i={:?},j={:?}]",
            m, robot.0, robot.1
        );
        let ni = robot.0 as i32 + m.0;
        let nj = robot.1 as i32 + m.1;

        let nchar = grid[ni as usize][nj as usize];

        if nchar == '#' {
            continue;
        }

        if nchar == '.' {
            grid[robot.0][robot.1] = '.';
            robot = (ni as usize, nj as usize);
            grid[robot.0][robot.1] = '@';
            continue;
        }

        if nchar == 'O' {
            // find all connected boxes in line, and move entire section a distance
            todo!()
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 2028)
    }
}
