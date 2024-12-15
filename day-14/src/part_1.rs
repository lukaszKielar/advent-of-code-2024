use std::collections::{HashMap, HashSet};

use crate::{parse_input, X, Y};

pub fn process(input: &str) -> usize {
    let robots = parse_input(input);

    let mut map: HashMap<u8, usize> = HashMap::new();
    map.entry(1).or_insert(0);
    map.entry(2).or_insert(0);
    map.entry(3).or_insert(0);
    map.entry(4).or_insert(0);

    for mut robot in robots {
        robot.move_sec(100, &(X, Y));

        let ending_position = robot.p;

        if (ending_position.0 >= 0 && ending_position.0 <= X / 2)
            && (ending_position.1 >= 0 && ending_position.1 <= Y / 2)
        {
            map.entry(1).and_modify(|count| *count += 1);
        } else if (ending_position.0 > X / 2 && ending_position.0 <= X)
            && (ending_position.1 >= 0 && ending_position.1 <= Y / 2)
        {
            map.entry(2).and_modify(|count| *count += 1);
        } else if (ending_position.0 >= 0 && ending_position.0 <= X / 2)
            && (ending_position.1 > Y / 2 && ending_position.1 <= Y)
        {
            map.entry(3).and_modify(|count| *count += 1);
        } else {
            map.entry(4).and_modify(|count| *count += 1);
        }
    }

    println!("{map:?}");

    let res = map.values().fold(1, |acc, &x| acc * x);
    res
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        ""
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        assert_eq!(X / 2, 2);
        let output = process(input);

        // then
        assert_eq!(output, todo!())
    }
}
