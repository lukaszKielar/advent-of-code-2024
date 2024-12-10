use crate::{parse_input, Coords};

pub fn process(input: &str) -> usize {
    let topo_map = parse_input(input);
    let mut res: usize = 0;

    for i in 0..topo_map.len() {
        for j in 0..topo_map[0].len() {
            if topo_map[i][j] != 0 {
                continue;
            }
            let mut local_res: usize = 0;

            let mut stack: Vec<Coords> = vec![];

            stack.push((i, j));

            while stack.len() > 0 {
                let (current_i, current_j) = stack.pop().unwrap();

                if topo_map[current_i][current_j] == 9 {
                    local_res += 1;
                    continue;
                }

                if current_i > 0 {
                    if topo_map[current_i - 1][current_j] == topo_map[current_i][current_j] + 1 {
                        stack.push((current_i - 1, current_j));
                    }
                }

                if current_j > 0 {
                    if topo_map[current_i][current_j - 1] == topo_map[current_i][current_j] + 1 {
                        stack.push((current_i, current_j - 1));
                    }
                }

                if current_i + 1 < topo_map.len() {
                    if topo_map[current_i + 1][current_j] == topo_map[current_i][current_j] + 1 {
                        stack.push((current_i + 1, current_j));
                    }
                }

                if current_j + 1 < topo_map[0].len() {
                    if topo_map[current_i][current_j + 1] == topo_map[current_i][current_j] + 1 {
                        stack.push((current_i, current_j + 1));
                    }
                }
            }

            res += local_res;
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
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 81)
    }
}
