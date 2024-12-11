fn blink(stone: &str) -> Vec<String> {
    if stone == "0" {
        return vec!["1".to_string()];
    } else if stone.len() % 2 == 0 {
        let left_stone = stone[..stone.len() / 2].to_string();
        let right_stone = stone[stone.len() / 2..]
            .parse::<usize>()
            .unwrap()
            .to_string();
        return vec![left_stone, right_stone];
    } else {
        return vec![(stone.parse::<usize>().unwrap() * 2024).to_string()];
    }
}

pub fn process(input: &str) -> usize {
    let mut stones = input
        .trim()
        .split_whitespace()
        .map(|elem| elem.to_string())
        .collect::<Vec<_>>();
    let mut blinks = 25;

    while blinks > 0 {
        println!("{blinks}");
        stones = stones.iter().flat_map(|s| blink(s)).collect::<Vec<_>>();
        blinks -= 1;
    }

    stones.len()
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
    #[case("125", vec!["253000"])]
    #[case("17", vec!["1", "7"])]
    #[case("1", vec!["2024"])]
    #[case("253000", vec!["253", "0"])]
    #[case("24", vec!["2", "4"])]
    fn test_blink(#[case] stone: &str, #[case] expected: Vec<&str>) {
        // when
        let output = blink(stone);

        // then
        assert_eq!(output, expected);
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 55312)
    }
}
