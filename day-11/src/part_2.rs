use std::collections::HashMap;

pub fn process(input: &str) -> usize {
    let stones = input
        .trim()
        .split_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut stones_map: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        stones_map.entry(stone).and_modify(|x| *x += 1).or_insert(1);
    }

    for _ in 0..75 {
        let mut new_stones_map = HashMap::new();

        for (stone, count) in stones_map {
            if stone == 0 {
                new_stones_map
                    .entry(1)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let half = stone_str.len() / 2;
                let left_stone = stone_str[..half].parse::<usize>().unwrap();
                let right_stone = stone_str[half..].parse::<usize>().unwrap();

                new_stones_map
                    .entry(left_stone)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
                new_stones_map
                    .entry(right_stone)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            } else {
                new_stones_map
                    .entry(stone * 2024)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            }
        }

        stones_map = new_stones_map;
    }

    stones_map.values().sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "125 17"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 65601038650482)
    }
}
