use std::sync::{atomic::AtomicUsize, Arc};

use crate::{parse_input, Coords, TopoMap, DIRECTIONS};

fn find_trailheads(
    coords: Coords,
    topo_map: &TopoMap,
    trailheads: Arc<AtomicUsize>,
    path: Vec<i32>,
) -> Arc<AtomicUsize> {
    let topo_map_value = topo_map[coords.0 as usize][coords.1 as usize];

    for direction in DIRECTIONS {
        let next_coords = (coords.0 + direction.0, coords.1 + direction.1);
        println!(
            "{:?}: current coords - [{:?},{:?}], next coords - [{:?},{:?}]",
            trailheads, coords.0, coords.1, next_coords.0, next_coords.1
        );

        if next_coords.0 < 0
            || next_coords.0 >= topo_map.len() as i32
            || next_coords.1 < 0
            || next_coords.1 >= topo_map[0].len() as i32
        {
            println!("\t{:?}: next point is out of bounds, skipping", trailheads);
            continue;
        }

        let next_topo_map_value = topo_map[next_coords.0 as usize][next_coords.1 as usize];
        let mut path = path.clone();

        if next_topo_map_value == 9 && path.len() == 9 {
            path.push(next_topo_map_value);
            println!(
                "\t{:?}: we have reached end of our trail: {:?}, exiting...",
                trailheads, path
            );
            trailheads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            continue;
        }

        if topo_map_value + 1 == next_topo_map_value {
            println!("\t{:?}: we have a match, following", trailheads);
            path.push(next_topo_map_value);
            find_trailheads(next_coords, topo_map, trailheads.clone(), path);
        }
    }

    trailheads
}

pub fn process(input: &str) -> usize {
    let topo_map = parse_input(input);
    let trailheads = Arc::new(AtomicUsize::new(0));

    for i in 0..topo_map.len() {
        for j in 0..topo_map[0].len() {
            if topo_map[i][j] == 0 {
                println!("I'm zero: topo_map[{i},{j}]");
                find_trailheads(
                    (i as i32, j as i32),
                    &topo_map,
                    trailheads.clone(),
                    vec![topo_map[i][j]],
                );
            }
        }
    }

    trailheads.load(std::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[rstest]
    fn test_find_trailheads() {
        // given
        let input = "1110111
1111111
1112111
6543456
7111117
8111118
9111119";
        let topo_map = &parse_input(input);

        // when
        let output = find_trailheads((0, 3), &topo_map, Arc::new(AtomicUsize::new(0)), vec![0]);
        let output = output.load(std::sync::atomic::Ordering::SeqCst);

        // then
        assert_eq!(output, 2)
        // assert_eq!(
        //     output,
        //     vec![
        //         [
        //             (0, 3),
        //             (1, 3),
        //             (2, 3),
        //             (3, 3),
        //             (3, 4),
        //             (3, 5),
        //             (3, 6),
        //             (4, 6),
        //             (5, 6),
        //             (6, 6),
        //         ],
        //         [
        //             (0, 3),
        //             (1, 3),
        //             (2, 3),
        //             (3, 3),
        //             (3, 2),
        //             (3, 1),
        //             (3, 0),
        //             (4, 0),
        //             (5, 0),
        //             (6, 0),
        //         ]
        //     ]
        // )
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 36)
    }
}
