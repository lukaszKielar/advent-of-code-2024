pub mod part_1;
pub mod part_2;

// (char, id)
type Block = (char, usize);

fn convert_to_blocks(input: &str) -> String {
    let mut output = String::new();
    let mut id = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap();
        for _ in 0..num {
            if i % 2 == 0 {
                output.push_str(&id.to_string().parse::<usize>().unwrap().to_string());
            } else {
                output.push('.');
            }
        }

        if i % 2 == 0 {
            id += 1;
        }
    }

    output
}

fn move_blocks_around(input: &str) -> String {
    let mut input = input.chars().collect::<Vec<_>>();

    for i in 0..input.len() {
        let mut j = input.len() - 1;
        match input[i] {
            '.' => {
                while j > i {
                    match input[j] {
                        '.' => {
                            j -= 1;
                        }
                        _ => {
                            input[i] = input[j];
                            input[j] = '.';
                            break;
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    String::from_iter(input)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "2333133121414131402"
    }

    #[rstest]
    fn test_convert_to_blocks(input: &str) {
        // when
        let output = convert_to_blocks(input);

        // then
        assert_eq!(output, "00...111...2...333.44.5555.6666.777.888899")
    }

    #[rstest]
    fn test_move_blocks_around(input: &str) {
        // given
        let blocks = convert_to_blocks(input);

        // when
        let output = move_blocks_around(&blocks);

        // then
        assert_eq!(output, "0099811188827773336446555566..............")
    }
}
