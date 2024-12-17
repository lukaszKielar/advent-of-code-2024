use crate::{get_robot_position, parse_input, Point};

pub fn process(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input);
    let mut robot = get_robot_position(&grid);
    let mut res = 0;

    for m in moves {
        let next_position: Point = (robot.i + m.i, robot.j + m.j).into();
        let next_char = grid[next_position.i as usize][next_position.j as usize];
        println!(
            "robot [x={:?},y={:?}], move: {:?}, next_char: [{:?}]",
            robot.i, robot.j, m, next_char
        );

        if grid[next_position.i as usize][next_position.j as usize] == '#' {
            continue;
        } else if grid[next_position.i as usize][next_position.j as usize] == '.' {
            grid[robot.i as usize][robot.j as usize] = '.';
            robot = next_position;
            grid[robot.i as usize][robot.j as usize] = '@';
        } else if grid[next_position.i as usize][next_position.j as usize] == 'O' {
            let mut next_next_position: Point =
                (next_position.i + m.i, next_position.j + m.j).into();

            while grid[next_next_position.i as usize][next_next_position.j as usize] == 'O' {
                next_next_position =
                    (next_next_position.i + m.i, next_next_position.j + m.j).into();
            }

            if grid[next_next_position.i as usize][next_next_position.j as usize] == '#' {
                continue;
            } else if grid[next_next_position.i as usize][next_next_position.j as usize] == '.' {
                grid[next_next_position.i as usize][next_next_position.j as usize] = 'O';
                grid[robot.i as usize][robot.j as usize] = '.';
                robot = next_position;
                grid[robot.i as usize][robot.j as usize] = '@';
            }
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                res += 100 * i + j
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
