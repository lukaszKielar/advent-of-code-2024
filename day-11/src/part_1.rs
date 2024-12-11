fn blink(stones: Vec<String>) -> Vec<String> {
    let mut output = vec![];

    for stone in stones {
        if stone == "0" {
            output.push("1".to_string());
            continue;
        }

        if stone.len() % 2 == 0 {
            let left_stone = stone[..stone.len() / 2].to_string();
            let right_stone = stone[stone.len() / 2..]
                .parse::<usize>()
                .unwrap()
                .to_string();
            output.push(left_stone);
            output.push(right_stone);
            continue;
        }

        let stone = (stone.parse::<usize>().unwrap() * 2024).to_string();
        output.push(stone);
    }

    output
}

pub fn process(input: &str) -> usize {
    let mut stones = input
        .trim()
        .split_whitespace()
        .map(|elem| elem.to_string())
        .collect();
    let mut blinks = 25;

    while blinks > 0 {
        stones = blink(stones);
        blinks -= 1;
    }

    stones.iter().count()
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
    fn test_blink() {
        // given
        let stones = vec!["125".to_string(), "17".to_string()];

        // when
        let output = blink(stones);
        assert_eq!(output, vec!["253000", "1", "7"]);

        let output = blink(output);
        assert_eq!(output, vec!["253", "0", "2024", "14168"]);

        let output = blink(output);
        assert_eq!(output, vec!["512072", "1", "20", "24", "28676032"]);

        let output = blink(output);
        assert_eq!(
            output,
            vec!["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"]
        );

        let output = blink(output);
        assert_eq!(
            output,
            vec![
                "1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60",
                "32"
            ]
        );

        let output = blink(output);
        assert_eq!(
            output,
            vec![
                "2097446912",
                "14168",
                "4048",
                "2",
                "0",
                "2",
                "4",
                "40",
                "48",
                "2024",
                "40",
                "48",
                "80",
                "96",
                "2",
                "8",
                "6",
                "7",
                "6",
                "0",
                "3",
                "2"
            ]
        );
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 55312)
    }
}
