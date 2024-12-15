use std::collections::HashMap;

use crate::parse_input;

pub fn process(input: &str, x: usize, y: usize) -> usize {
    let robots = parse_input(input);

    let mut quadrant_map: HashMap<u8, usize> = HashMap::new();
    quadrant_map.entry(0).or_insert(0);
    quadrant_map.entry(1).or_insert(0);
    quadrant_map.entry(2).or_insert(0);
    quadrant_map.entry(3).or_insert(0);

    let quadrant_height = (y - 1) / 2;
    let quadrant_width = (x - 1) / 2;

    for mut robot in robots {
        robot.move_sec(100, &(x, y));
        println!("{robot:?}");

        let ending_position = robot.p;

        if ending_position.0 > quadrant_width {
            if ending_position.1 > quadrant_height {
                quadrant_map.entry(3).and_modify(|count| *count += 1);
            } else if ending_position.1 < quadrant_height {
                quadrant_map.entry(1).and_modify(|count| *count += 1);
            }
        } else if ending_position.0 < quadrant_width {
            if ending_position.1 > quadrant_height {
                quadrant_map.entry(2).and_modify(|count| *count += 1);
            } else if ending_position.1 < quadrant_height {
                quadrant_map.entry(0).and_modify(|count| *count += 1);
            }
        }
    }

    println!("{quadrant_map:?}");

    let res = quadrant_map.values().fold(1, |acc, &x| acc * x);
    res
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
        let output = process(input, 11, 7);

        // then
        assert_eq!(output, 12)
    }
}
