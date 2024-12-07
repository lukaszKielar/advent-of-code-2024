use std::collections::HashMap;

#[cfg(test)]
use crate::pretty_print;
use crate::{get_starting_point, move_guard_around, parse_input, Coords, Matrix, Point, OBSTACLE};

fn is_valid_obstacle(matrix: &Matrix, visited_coords: &Vec<Coords>, new_obstacle: Coords) -> bool {
    // place an obstacle in a position
    let mut iter: usize = 0;

    let mut is_valid: bool = false;

    let mut new_matrix = {
        let mut matrix = matrix.clone();
        matrix[new_obstacle.0][new_obstacle.1] = OBSTACLE;
        matrix
    };

    #[cfg(test)]
    pretty_print(&new_matrix);

    let mut coords_count: HashMap<Coords, usize> = HashMap::new();
    let mut current_point = get_starting_point(&matrix);
    let mut last_8_obstacles: Vec<Coords> = Vec::new();
    let mut next_point_coords = current_point.next_point_coords();

    // mark point as visited and add it to count hashmap
    new_matrix[current_point.coords.0][current_point.coords.1] = 'X';
    coords_count
        .entry(current_point.coords)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);

    // loop as long as:
    // 1. guard follows original path AND
    // 2. stays within bounds of the matrix
    while (next_point_coords.0 <= new_matrix.len() - 1)
        && (next_point_coords.1 <= new_matrix[0].len() - 1)
        && (visited_coords.contains(&next_point_coords)
            || matrix[next_point_coords.0][next_point_coords.1] == OBSTACLE)
    {
        if iter > 0 {
            // mark point as visited and add it to count hashmap
            new_matrix[current_point.coords.0][current_point.coords.1] = 'X';
            coords_count
                .entry(current_point.coords)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        // breaks when we discover the loop
        // FIXME: this condition doesn't work as expected, just a placeholder for now
        // I should probably collect only locations where I make a turn
        // and break from the loop when last 8 elements 0
        // when I can find repetitive pattern, and keep last 8 items
        // if last_8_obstacles.len() == 8
        //     && last_8_obstacles[0] == last_8_obstacles[4]
        //     && last_8_obstacles[1] == last_8_obstacles[5]
        //     && last_8_obstacles[2] == last_8_obstacles[6]
        //     && last_8_obstacles[3] == last_8_obstacles[7]
        if coords_count.values().all(|&elem| elem >= 2) || iter >= 200_000 {
            is_valid = true;

            #[cfg(test)]
            println!("we have a loop! exiting...");

            break;
        }

        if new_matrix[next_point_coords.0][next_point_coords.1] == OBSTACLE {
            let updated_direction = current_point.direction.turn_right();
            current_point.direction = updated_direction;
            // last_8_obstacles.push(next_point_coords);
            // if last_8_obstacles.len() >= 8 {
            //     let new_last_8_obstacles = last_8_obstacles[last_8_obstacles.len() - 8..].to_vec();
            //     last_8_obstacles = new_last_8_obstacles;
            // }
        };

        #[cfg(test)]
        pretty_print(&new_matrix);

        let new_point = Point {
            coords: current_point.next_point_coords(),
            ..current_point
        };
        current_point = new_point;
        next_point_coords = current_point.next_point_coords();

        iter += 1;
    }

    is_valid
}

fn get_visited_coords(matrix: &Matrix) -> Vec<Coords> {
    let mut visited_coords = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                visited_coords.push((i, j));
            }
        }
    }

    visited_coords
}

pub fn process(input: &str) -> usize {
    let matrix = parse_input(input);

    #[cfg(test)]
    println!("Input matrix");
    #[cfg(test)]
    pretty_print(&matrix);

    let point = get_starting_point(&matrix);
    let solved_matrix = move_guard_around(matrix.clone(), point.coords, point.direction);
    let visited_coords = get_visited_coords(&solved_matrix);

    #[cfg(test)]
    println!("Solved matrix");
    #[cfg(test)]
    pretty_print(&solved_matrix);

    #[cfg(test)]
    println!("Visited coords: {}", visited_coords.len());

    let mut res = 0;
    let mut iter = 0;
    for c in &visited_coords {
        iter += 1;
        println!("{}/{}", iter, visited_coords.len());
        if c.eq(&point.coords) {
            continue;
        }
        if is_valid_obstacle(&matrix, &visited_coords, c.to_owned()) {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::Point;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[fixture]
    fn matrix(input: &str) -> Matrix {
        parse_input(input)
    }

    #[fixture]
    fn starting_point(matrix: Matrix) -> Point {
        get_starting_point(&matrix)
    }

    #[fixture]
    fn solved_matrix(matrix: Matrix, starting_point: Point) -> Matrix {
        move_guard_around(matrix, starting_point.coords, starting_point.direction)
    }

    #[fixture]
    fn visited_coords(solved_matrix: Matrix) -> Vec<Coords> {
        get_visited_coords(&solved_matrix)
    }

    #[rstest]
    fn test_get_visited_coords(solved_matrix: Matrix) {
        // when
        let output = get_visited_coords(&solved_matrix);

        // then
        assert_eq!(
            output,
            [
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (2, 4),
                (2, 8),
                (3, 4),
                (3, 8),
                (4, 2),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
                (4, 8),
                (5, 2),
                (5, 4),
                (5, 6),
                (5, 8),
                (6, 2),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
                (6, 7),
                (6, 8),
                (7, 1),
                (7, 2),
                (7, 3),
                (7, 4),
                (7, 5),
                (7, 6),
                (7, 7),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (8, 6),
                (8, 7),
                (9, 7)
            ]
        )
    }

    #[rstest]
    // #[case((6,3))]
    #[case((7,6))]
    #[case((7,7))]
    #[case((8,1))]
    #[case((8,3))]
    #[case((9,7))]
    fn test_is_valid_obstacle_truthy(
        matrix: Matrix,
        visited_coords: Vec<Coords>,
        #[case] new_obstacle: Coords,
    ) {
        // when
        let output = is_valid_obstacle(&matrix, &visited_coords, new_obstacle);

        // then
        assert!(output)
    }

    #[rstest]
    #[case((1,4))]
    #[case((1,8))]
    #[case((4,6))]
    #[case((8,2))]
    #[case((8,4))]
    #[case((6,5))]
    fn test_is_valid_obstacle_falsy(
        matrix: Matrix,
        visited_coords: Vec<Coords>,
        #[case] new_obstacle: Coords,
    ) {
        // when
        let output = is_valid_obstacle(&matrix, &visited_coords, new_obstacle);

        // then
        assert!(!output)
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 6)
    }
}
